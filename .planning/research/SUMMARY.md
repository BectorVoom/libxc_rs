# Project Research Summary

**Project:** libxc_rs  
**Domain:** Rust CubeCL-based re-architecture of libxc’s public API  
**Researched:** 2026-03-22  
**Confidence:** MEDIUM

## Executive Summary
libxc_rs is a from-scratch rewrite of the libxc 7.0.0 public surface that trades the original C API for a three-layer Rust interface (compat, safe core, ergonomic) while routing every numerical execution path—including CPU—through CubeCL kernels. Experts build this kind of product by pairing generated inventories and metadata with strict validation so the API surface stays complete, safe, and aligned with the upstream oracle.

The research recommends a workflow that starts with deterministic code generation (bindgen + xtask pipelines), layers generated metadata under type-safe builders, and then sprints toward the CubeCL substrate so both CPU and GPU share one kernel family. Resident execution, workspace reuse, and verification harnesses wrap that kernel layer to keep long-running workloads efficient and easy to trust.

Key risks remain coverage drift, backend divergence, and verification gaps. Automating API/catalog generation and regression-checking the ID counts guards the inventory, sharing the CubeCL kernel substrate plus parity suites keeps CPU/GPU semantics aligned, and baking the oracle harness and benchmarking into the roadmap prevents unnoticed regressions.

## Key Findings

### Recommended Stack
The stack reinforces deterministic generation (bindgen/xtask) for the libxc inventory, CubeCL 0.9.0 for unified compute, and typed error/host tooling (`thiserror`/`anyhow`, `ndarray`, `rayon`) to keep validation plus preprocessing high quality.

**Core technologies:**
- **CubeCL compute stack 0.9.0:** single-kernel substrate shared by CPU and GPU runtimes keeps formulas consistent while CubeCL handles autotuning and backend launches.
- **bindgen 0.72.0 / cargo xtask pattern:** auto-generates the hundreds of functions, IDs, and metadata tables and keeps the parser/codegen pipeline orchestrated from a repeatable xtask script.
- **thiserror 2.0 + anyhow 1.0.102:** typed public errors and context-rich tooling errors maintain clear boundaries between the safe API and verification/xtask helpers.

### Expected Features
The feature research confirms that reachability to the full libxc inventory, a layered API, and CubeCL-powered execution are table stakes, while generated dispatch tables, typed builders, and resident optimizations are differentiators that amplify the value of the safe API surface.

**Must have (table stakes):**
- Generated coverage of all 85 functions / 649 IDs + legacy handling — the library must make every upstream entry reachable.
- Layered API (compat handles + safe core + ergonomic builders/resident APIs) — Rust callers expect ergonomics without sacrificing raw entry points.
- Unified CubeCL execution path for CPU/GPU + strict validation — prevents semantic drift and unsafe launches.

**Should have (competitive):**
- Single CubeCL kernel family with shared primitives + generated dispatch tables — keeps CPU and GPU semantics aligned while letting the runtime pick the right specialization.
- Typed builders/batch/resident APIs with explicit runtime policy controls — makes complex launches predictable and scalable.

**Defer (v2+):**
- Expanded autotuning/policy controls and new functional families (LCA/OEP) — postpone until dispatch stability and CubeCL coverage are proven.

### Architecture Approach
The architecture layers a public API over generated metadata, validation models, and execution coordination so callers never touch unsafe pointers; evaluation work flows through workspace planners, runtime adapters, and kernel modules before feeding back masked outputs to the API surface.

**Major components:**
1. `api::builder`, `api::functional`, `api::resident` — provide typed entry points, lifecycle management, and resident buffer orchestration.
2. `meta`, `registry`, `model`, `layout` — host generated metadata, validation predicates, and layout/shape helpers that guard every dispatch.
3. `eval`, `workspace`, `runtime`, `kernel` — coordinate buffer planning, CubeCL backend selection, and shared kernels for family/order/precision combinations.

### Critical Pitfalls
1. **Generated API coverage drift** — automate bindgen/xtask runs, lock tool versions, and treat inventory counts as regression checks before releases.  
2. **CPU/GPU semantic divergence** — share the CubeCL kernel substrate, run parity suites (CPU/CUDA/HIP/WGPU) per derivative order, and surface divergence in resident/verification flows.  
3. **Verification gaps** — ship the oracle harness (Phase 7) and nightly parity/abs-rel/ULP suites so each API path is continuously compared to libxc.

## Implications for Roadmap
Suggested phase structure:

### Phase 1: Inventory + Metadata Generation
**Rationale:** API completeness depends on deterministic codegen before any runtime work begins.  
**Delivers:** Generated registries (functions, IDs, removed entries), metadata tables, and the xtask pipeline.  
**Addresses:** Generated inventory coverage, registry lookups, and compat layer readiness.  
**Avoids:** Pitfall 1 (coverage drift) by locking tool versions and counting IDs.

### Phase 2: Validation + Shapes
**Rationale:** Layout, input bundles, and workspace planning must compile against the metadata so evaluation layers can rely on shape guarantees.  
**Delivers:** `layout`, `input`, `output`, and `workspace/planner` modules plus shape/threshold validators.  
**Addresses:** Typed validation (families, spins, taus) and workspace reuse preparation.  
**Avoids:** Pitfall data-starvation by allowing dirty-range tracking before kernel launches.

### Phase 3: CubeCL Kernel Substrate
**Rationale:** With metadata/validation ready, build the shared CubeCL kernel helpers so kernels can be compiled once for all backends.  
**Delivers:** `kernel/shared` utilities, CubeCL threshold/spin/ext-param helpers, and capability-probed runtimes.  
**Uses:** CubeCL 0.9.0 stack + runtime cache patterns.  
**Implements:** Kernel/resolution layering from the architecture doc.  
**Avoids:** Pitfall 2 (CPU/GPU divergence) by collapsing logic into the shared substrate and gating backends via capability probes.

### Phase 4: Resident Execution & Safe API
**Rationale:** After kernels exist, stabilize resident flows, layering, and ergonomics so end users can evaluate CPU/GPU workloads safely.  
**Delivers:** Resident workspace caches, typed builder/functional APIs, compat shims, and worker runtime selection.  
**Addresses:** Resident execution buffers, output masking, typed builders, and compat surfaces.  
**Avoids:** Pitfall 5 (transfer costs) via dirty-range-aware workspace planners and masked outputs; uses `ndarray` + `rayon` prep to keep host work cheap.

### Phase 5: Verification & Benchmarking
**Rationale:** Trust requires full oracle comparisons plus benchmark baselines before declaring readiness.  
**Delivers:** Verification harness (abs/rel/ULP reports, CPU/GPU parity), Criterion benchmarks for lookup/init/transfer, and runtime cache tuning.  
**Addresses:** Verification suite, benchmarking, and resident throughput metrics.  
**Avoids:** Pitfall 3 (verification gaps) and surfaces transfer traps via profiling dashboards.

### Phase Ordering Rationale
- Codegen/metadata must precede validation because the stack relies on generated tables (Phase 1 → Phase 2).  
- Validation and workspace setup must finish before compiling CubeCL kernels so launch arguments are shaped correctly (Phase 2 → Phase 3).  
- The shared kernel substrate enables resident APIs, so Phase 3 naturally feeds Phase 4, which in turn feeds Phase 5’s verification/benchmarks.

### Research Flags
Phases likely needing deeper research during planning:
- **Phase 3:** CubeCL CPU runtime maturity, capability probing, and backend asymmetries require tech validation before committing to specific launch paths.  
- **Phase 5:** Verification harness scope (order/family/runtime matrix) and benchmark stabilization need detailed acceptance criteria.

Phases with standard patterns:
- **Phase 1:** Codegen and inventory generation follow well-documented xtask/bindgen practices described in the design doc.  
- **Phase 2:** Shape validation and workspace planning align with existing layout + workspace patterns, so minimal extra research is needed.

## Confidence Assessment
| Area | Confidence | Notes |
|------|------------|-------|
| Stack | MEDIUM | Derived from STACK.md research with documented versions and alternatives. |
| Features | HIGH | Features are traced directly to requirements in FEATURES.md and the detailed design. |
| Architecture | MEDIUM | Structure comes from ARCHITECTURE.md and the detailed design, which align but still need implementation experience. |
| Pitfalls | MEDIUM | PITFALLS.md calls out known traps; mitigation is clear but depends on future verification work. |

**Overall confidence:** MEDIUM

### Gaps to Address
- **CubeCL CPU maturity & capability coverage:** Validate which operations the CPU runtime already supports and how unsupported cases are surfaced before leaning on that backend.  
- **Transfer/resident heuristics & provisional thresholds:** Tune dirty-range uploads, output masking, and provisional thresholds (especially `kxc`/`lxc`) once the kernels and resident planner exist.

## Sources
### Primary (HIGH confidence)
- `docs/libxc_rs_detailed_design.md` — foundational design, architecture breakdown, inventory counts, and phased implementation plan.

### Secondary (MEDIUM confidence)
- `.planning/research/STACK.md` — tooling, dependency versions, and stack rationale.  
- `.planning/research/FEATURES.md` — feature priorities, dependencies, and MVP definition.  
- `.planning/research/ARCHITECTURE.md` — layer map, data flow, and major component responsibilities.  
- `.planning/research/PITFALLS.md` — top pitfalls, mitigation advice, and phase mapping.  
- `.planning/PROJECT.md` — requirements, constraints, and context for the rewrite.

### Tertiary (LOW confidence)
- None — no low-confidence sources were used.

---
*Research completed: 2026-03-22*  
*Ready for roadmap: yes*
