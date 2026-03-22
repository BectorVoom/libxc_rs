# Architecture Research

**Domain:** Rust CubeCL-based libxc re-architecture
**Researched:** 2026-03-22
**Confidence:** MEDIUM

## Standard Architecture

### System Overview

```
┌──────────────────────────────────────────────────────────────┐
│                    Public API Surface Layer                   │
├──────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │ api::builder │  │ api::functional │  │ api::resident │        │
│  └────┬─────────┘  └────┬─────────┘  └────┬─────────┘        │
│       │               │               │                      │
├───────┴───────────────┴───────────────┴──────────────────────┤
│                   Safe + Compat Facade Layer                  │
├──────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │ meta/registry│  │ model/layout │  │ error/public │        │
│  └────┬─────────┘  └────┬─────────┘  └────┬─────────┘        │
│       │               │               │                      │
├───────┴───────────────┴───────────────┴──────────────────────┤
│                    Execution Coordination Layer               │
├──────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐           │
│  │ eval/*      │  │ workspace/* │  │ runtime/*   │           │
│  └────┬────────┘  └────┬────────┘  └────┬────────┘           │
│       │               │               │                      │
├───────┴───────────────┴───────────────┴──────────────────────┤
│                    Kernel & Generated Data Layer              │
│  ┌────────────┐  ┌────────────┐  ┌──────────────────────┐   │
│  │ kernel/*   │  │ generated/ │  │ verification/benches │   │
│  └────────────┘  └────────────┘  └──────────────────────┘   │
└──────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| `api::builder` | Constructs typed `Functional` handles with spin, thresholds, precision, and runtime policies; validates ID/name choices | Builder pattern returning `Functional` plus typed errors (`thiserror` v2) |
| `api::functional` | Holds validated metadata + mutable thresholds/ext-params + runtime binding; exposes evaluation variants (single-point, batch, resident) | Encapsulates `eval::dispatcher` and runtime policies to launch CubeCL workflows |
| `api::resident` | Hosts resident buffers, dirty-range optimization, and selective output masking | Wraps `workspace::resident` with borrow-checked device buffers |
| `meta` / `registry` / `model` | Houses generated functional metadata, ID/name maps, family/kind/spin/flags models | Generated tables + derived helpers for cost-free lookups and validation predicates |
| `layout` | Validates shapes and encodes packed/strided/SoA layout info for each family/order/spin | Mirrors `xc_dimensions`, carries stride heuristics for CubeCL kernels |
| `workspace` | Plans and reuses host/device scratch for input transforms, accumulation, and output staging | Cache keyed by runtime/family/order to avoid repeated allocations |
| `runtime` | Adapts CubeCL CPU/CUDA/HIP/WGPU backends with capability probing and launch cache | Wraps CubeCL device selection, stream management, and runtime caching |
| `eval` | Validates user request, prepares buffers, dispatches CubeCL kernels, and finalizes readbacks/masks | Divides into dispatch, prepare, execute, finalize, and policy to keep launch path deterministic |
| `kernel` | Contains shared helpers and family/order-specific CubeCL kernels for LDA/GGA/MGGA plus mixing | Single kernel substrate shared across CPU/GPU, with `shared` helpers for thresholds, spin, ext params |
| `generated` | Stores artifacts from `xtask` (registry, legacy IDs, ext-param specs, dispatch tables) required for metadata + kernel specialization | Re-generated via build script before compilation |
| `verification/benches` | Compare CubeCL results vs libxc oracle and measure lookup/launch/transfer performance | Standalone Cargo targets referencing `verify/` and `benches/` folders |

## Recommended Project Structure

```
src/
├── api/                 # public surface (builder, functional handles, resident APIs)
├── compat/              # raw handle compatibility layer and C-layout adapters
├── error/               # public/internal/FFI error mapping
├── eval/                # dispatch/prepare/execute/finalize policy for CubeCL launches
├── kernel/              # shared CubeCL helpers + LDA/GGA/MGGA/mix kernels
├── layout/              # dimension formulas, packed/strided/SoA layout descriptions
├── meta/                # metadata descriptors, ext params, hybrid/NLC graphs
├── model/               # enums for family/kind/spin/derivative/flags/precision
├── registry/            # generated constant-time registries, alias handling
├── runtime/             # CubeCL runtime adapters, capability cache, streams
├── workspace/           # scratch planner + host/resident buffer caches
└── generated/           # build-time artifacts produced by xtask
```

### Structure Rationale
- **`api/` + `compat/`** keep the three-layer surface (safe core + ergonomic resident API + compat shim) aligned with the documented requirements.
- **`meta/`, `registry/`, `model/`** centralize inventory/state so lookups remain O(1) and validation rules share a single truth source.
- **`eval/`, `workspace/`, `runtime/`, `kernel/`** manifest the execution pipeline: validate → plan buffers → select CubeCL runtime → launch shared kernels.
- **`generated/` + `xtask/`** reflect build-order exposure: registries and dispatch tables must exist before evaluation code compiles; generation precedes consumer crates.

## Architectural Patterns

### Pattern 1: Layered API with Compatibility Shim
**What:** Separate builder/safe API, compat raw handles, and ergonomic resident entry points so callers can choose the comfort/performance trade-off.
**When to use:** Supporting legacy binaries while modernizing the public surface.
**Trade-offs:** Slight code duplication between layers; but keeps typed safety front-and-center.

**Example:**\n```rust
let func = FunctionalBuilder::new(FunctionalId::PBE0)
    .spin(Spin::Unpolarized)
    .thresholds(Thresholds::default())
    .build()?;
func.evaluate(&input).await?;
```

### Pattern 2: Generated Registry + Dispatch Tables
**What:** Use `xtask` to parse `libxc` headers and emit metadata/dispatch artifacts included at compile time.
**When to use:** When the domain requires complete coverage (85 functions, 649 IDs, legacy/removed) without manual drift.
**Trade-offs:** Build step adds complexity, but runtime performance and correctness depend on it.

### Pattern 3: Resident Execution Cache
**What:** Keep device-resident workspace buffers keyed by runtime/family/order with dirty-range tracking.
**When to use:** Workloads reuse the same functional repeatedly, especially for GPU runs.
**Trade-offs:** Needs explicit invalidation, but avoids repeated host-device transfers and CubeCL re-allocations.

## Data Flow

### Request Flow
```
[Caller] → api::builder → api::functional/resident
    ↓                    ↓
  validate (model/registry/layout)
    ↓                    ↓
  eval::prepare → workspace (scratch + resident buffers)
    ↓
  runtime selection (CubeCL CPU/CUDA/HIP/WGPU)
    ↓
  kernel::launch → CubeCL kernel + shared helpers
    ↓
  eval::finalize → mask outputs/readback → [Caller]
```

### State Management
```
[Runtime Cache] ←── workspace::resident ──→ [CubeCL buffers]
     ↑              (dirty-range tracking)      ↑
     └────────────eval::prepare───────────────┘
```

### Key Data Flows
1. **Metadata → Validation:** Registry metadata feeds `model::feature_requirements` and `layout::validation` before any CubeCL dispatch to ensure thresholds/spin/derivative requests match capabilities.
2. **Caller Inputs → CubeCL Kernels:** Borrowed or resident input bundles are transformed into SoA buffers by `workspace::planner` and passed to `kernel::shared` structs for uniform CPU/GPU execution.
3. **Runtime → Resident Outputs:** `eval::finalize` reads masked outputs, updates resident buffers if requested, and surfaces typed bundles back to the API layer.

## Build-Order Implications
1. **Code Generation (`xtask`)** – parse headers/inventories to emit `generated/` registries and dispatch tables used across `meta/`, `registry/`, and `kernel/`.
2. **Metadata + Registry + Model Layers** – these must compile before evaluation code so validation helpers can reference constants.
3. **Layout + Input/Output + Workspace** – shape validation and buffer planning depend on metadata but are required before kernel launches exist.
4. **Kernel Substrate + CubeCL Runtimes** – build shared kernel helpers and runtime adapters (CPU/CUDA/HIP/WGPU) so `eval` can dispatch consistently.
5. **Per-Family Kernels + Dispatch Tables** – generate and compile LDA/GGA/MGGA/mix kernels specialized by order, output masks, and precision; required for resident/execution flows.
6. **Public API + Compat Exports** – wire safe handles and compat shims over the evaluation stack once lower layers are stable.
7. **Verification/Benchmarks** – run after runtime is functioning to compare CubeCL vs libxc and gather metrics.

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| 0-1k evaluations | Single-threaded CPU CubeCL runtime is sufficient; focus on correctness and metadata completeness. |
| 1k-100k evaluations | Enable resident execution, kernel cache, and stream parallelism in `runtime::streams`; emphasize workspace reuse. |
| 100k+ evaluations | Tune CubeCL kernel launch parameters, multi-stream workloads, and data placement for GPUs; monitor verification/bench reports for drift. |

### Scaling Priorities
1. **First bottleneck:** Workspace/cached buffer exhaustion; mitigate by improving `workspace::planner` heuristics and reuse maps.
2. **Second bottleneck:** CubeCL kernel dispatch latency; optimize via `runtime::cache` and per-family kernel specialization.

## Anti-Patterns

### Anti-Pattern 1: Separate CPU vs GPU Formulas
**What people do:** Maintain distinct handwritten CPU implementations alongside CubeCL GPU kernels.
**Why it's wrong:** Introduces semantic drift, doubles testing surface, contradicts the CubeCL-only requirement.
**Do this instead:** Share the CubeCL substrate across CPU and GPU runtimes (as enforced by `kernel` and `runtime`).

### Anti-Pattern 2: Lazy Metadata Generation at Runtime
**What people do:** Parse `libxc` headers on every startup to fill registries.
**Why it's wrong:** Breaks determinism, slows builds, and risks incomplete coverage.
**Do this instead:** Run `xtask` once (or as part of `build.rs`) to generate static tables consumed by the crate.

## Integration Points

### External Services
| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| CubeCL | Runtime adapters per backend (CPU/CUDA/HIP/WGPU) | All compute flows route through CubeCL kernels; capability gating handled in `runtime::capability`. |
| libxc (oracle) | Verification-only FFI via `verify/src/oracle_ffi.rs` | Only the verify harness links against libxc to compare accuracy; production crate never calls libxc. |

### Internal Boundaries
| Boundary | Communication | Notes |
|----------|---------------|-------|
| `api::*` ↔ `eval::*` | Direct Rust function calls | API layer orchestrates validation, dispatch, and finalization without sharing mutable global state. |
| `eval::*` ↔ `runtime::*` | Runtime selection + launch cache | `runtime::device` describes backend capabilities; `eval::execute` uses that to dispatch `kernel::launch`. |
| `workspace::*` ↔ `kernel::*` | Shared buffer descriptors + `shared` structs | Workspaces plan SoA buffers matching CubeCL layouts defined in `kernel/shared/types.rs`. |
| `meta/registry` ↔ `generated/dispatch_tables` | Metadata ↔ capability masks | Dispatch keys rely on generated tables to determine supported derivatives/spin combos. |

## Sources
- `docs/libxc_rs_detailed_design.md`
- `.planning/PROJECT.md`

---
*Architecture research for: Rust CubeCL-based libxc re-architecture*
*Researched: 2026-03-22*
