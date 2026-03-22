# libxc_rs

## What This Is

`libxc_rs` is a Rust re-architecture of the full public `libxc` API surface for the `libxc 7.0.0` inventory captured in `docs/libxc_rs_detailed_design.md`. It preserves reachability to libxc public capabilities, but replaces the original C-style surface with a layered Rust API: a low-level compatibility layer, a typed safe core, and an ergonomic high-level interface for host and device execution.

This project is explicitly designed around a unified CubeCL compute path. All numerical evaluation on CPU and GPU goes through CubeCL kernels, while libxc itself remains an oracle used only by verification tooling.

## Core Value

Deliver full libxc public capability coverage through a safer Rust API without splitting CPU and GPU semantics into separate evaluator implementations.

## Requirements

### Validated

(None yet — ship to validate)

### Active

- [ ] Reproduce the libxc public inventory in generated Rust registries, including all 85 public functions, 649 current public functional IDs, and explicit handling of legacy or removed identifiers.
- [ ] Expose a layered API surface that covers metadata, registry lookup, lifecycle, threshold and external-parameter control, typed family inputs and outputs, resident execution, and compatibility shims.
- [ ] Route all numerical execution through CubeCL on both CPU and GPU, with shared kernel logic for family, derivative-order, masking, and auxiliary accumulation behavior.
- [ ] Enforce type-safe validation for family, spin, derivative order, shapes, required MGGA channels, and public error handling before any kernel launch.
- [ ] Provide resident execution, buffer reuse, output masking, and launch/runtime caching to keep repeated evaluation paths efficient.
- [ ] Compare every supported evaluation path against libxc through a verification harness that reports abs/rel/ULP error metrics and CPU-vs-GPU parity.
- [ ] Track cold and warm performance behavior with focused benchmarks for lookup, initialization, host execution, resident execution, and transfer costs.

### Out of Scope

- Embedding libxc as the production evaluator — libxc is reserved for verification so the runtime stays Rust-native.
- Maintaining a separate handwritten CPU formula implementation — the design requires CubeCL-only numerical execution to avoid semantic drift.
- Freezing C ABI-compatible structs as the primary user-facing API — the compatibility layer exists for reachability, not as the main ergonomic surface.

## Context

- The supplied design document is the primary project specification and targets the public surface found in the inspected `libxc 7.0.0` source bundle.
- The project already has a Rust crate layout with modules under `src/`, generated-artifact tooling under `xtask/`, verification tooling under `verify/`, and tests and benchmarks under `tests/` and `benches/`.
- The design establishes a three-layer API: compatibility, safe core, and ergonomic high-level execution.
- The current architecture depends on build-time inventory/code generation for registries, metadata tables, removed-ID handling, and dispatch tables.
- CPU and GPU numerical execution must share one CubeCL kernel codebase, with runtime differences limited to device/runtime policy, launch configuration, and capability probing.
- Verification depends on an oracle harness that calls libxc only from isolated tooling, not from the production library runtime.

## Constraints

- **Tech stack**: Rust crate with CubeCL runtimes for CPU/CUDA/HIP/WGPU — all compute paths must share the same kernel substrate.
- **Compatibility**: Full reachability to the libxc public API inventory — the redesign cannot silently drop public functions, IDs, or metadata paths.
- **Safety**: Public APIs use typed Rust boundaries and `thiserror` v2 errors — unsafe code is confined to compat/raw-handle internals, CubeCL launch bridging, and verification FFI.
- **Validation**: libxc remains the oracle — acceptance requires oracle comparisons across family, derivative order, spin mode, and runtime combinations.
- **Performance**: Repeated workloads must reuse workspaces, resident buffers, and caches — hidden hot-path allocations and unnecessary transfers are not acceptable.
- **Scope discipline**: The design is a from-scratch public API redesign, not a transliteration of the original C header.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Use CubeCL for all numerical execution, including CPU | One kernel codebase reduces CPU/GPU drift and concentrates optimization effort | — Pending |
| Keep libxc out of the production runtime and use it only in verification tooling | Preserves a Rust-native runtime while still grounding correctness in the upstream oracle | — Pending |
| Structure the public surface in three layers: compat, safe core, ergonomic high-level API | Preserves complete reachability while giving Rust callers typed, safer abstractions | — Pending |
| Generate metadata, registry, removed-ID, and dispatch artifacts at build time | Inventory completeness and constant-time lookups depend on generated source artifacts rather than runtime parsing | — Pending |
| Treat resident execution and selective output materialization as first-class capabilities | GPU throughput and repeated CPU/GPU workloads depend on transfer minimization and output masking | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `$gsd-transition`):
1. Requirements invalidated? -> Move to Out of Scope with reason
2. Requirements validated? -> Move to Validated with phase reference
3. New requirements emerged? -> Add to Active
4. Decisions to log? -> Add to Key Decisions
5. "What This Is" still accurate? -> Update if drifted

**After each milestone** (via `$gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check -> still the right priority?
3. Audit Out of Scope -> reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-03-22 after initialization*
