# Feature Research

**Domain:** Rust rewrite of libxc with unified CubeCL compute
**Researched:** 2026-03-22
**Confidence:** HIGH

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist. Missing these = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Generated coverage of the entire libxc public API surface (85 functions, 649 current IDs, legacy and removed entries) | Project core value is preserving reachability to the upstream API so downstream users see no gaps | HIGH | Depends on xtask parsing pipelines and build-time registry/code generation described in docs |
| Layered API: compat raw handles, safe core validation, and ergonomic high-level builders/execution | Users expect Rust ergonomics while keeping legacy entry points reachable | MEDIUM | Drives module structure under `src/api`, `src/compat`, and the generated metadata tables |
| Unified CubeCL compute path for CPU and GPU workloads | Domain requires parity with libxc semantics without divergent CPU/GPU code | HIGH | All numerical launch logic (single-point, batch, resident, auxiliary) runs through CubeCL kernels per design |
| Type-safe validation for family/spin/derivative orders, thresholds, ext params, and shape/layouts before kernel launches | Prevents undefined behavior and mimics libxc validation in a Rust-friendly way | MEDIUM | Informs `model/*`, `layout/*`, and `eval/prepare.rs` validation helpers |
| Resident execution with buffer reuse, launch caching, and explicit output masking | Repeated workloads (e.g., SCF loops) need efficient host-device interaction | MEDIUM | Features are surfaced via `api/resident`, `workspace`, and kernel output-mask utilities |
| Verification harness and benchmarking against libxc oracle (abs/rel/ULP metrics plus CPU/GPU parity) | Users expect correctness guarantees and performance transparency | HIGH | Verification lives under `verify/` and `tests/` ensuring CubeCL matches libxc |

### Differentiators (Competitive Advantage)

Features that set the product apart. Not required, but valuable.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Single CubeCL kernel family for all families/orders/derivatives, including hybrid/aux accumulation | Eliminates drift between CPU & GPU, lets optimizations focus on one substrate | HIGH | Kernel directories (`kernel/lda`, `kernel/gga`, `kernel/mgga`, `kernel/mix`) all share `kernel/shared` primitives |
| Generated dispatch tables + metadata-driven registry lookups | Enables constant-time ID/name resolution with type-safe metadata for families, ext params, flags, etc. | MEDIUM | Depends on `generated/` artifacts and `registry/` helpers; also supports removed-ID diagnostics |
| Typed builder and batch APIs that declaratively select runtime policy, precision, and output masks | Makes complex libxc usage ergonomic while keeping validation explicit | MEDIUM | `api/builder.rs`, `api/batch.rs`, `api/functional.rs` coordinate selection, validation, and launch prep |
| Compatibility shims that offer legacy aggregate outputs, removed-ID diagnostics, and raw-handle migration hints | Helps existing C users move to Rust without dropping legacy macros | MEDIUM | `compat/legacy_eval.rs`, `compat/removed.rs`, and `compat/raw_handle.rs` implement this behavior |
| Resident scratch/workspace planner with dirty-range uploads and device scratch reuse | Minimizes allocations and transfer overhead for iterative simulations | MEDIUM | `workspace/planner.rs`, `workspace/host.rs`, and `workspace/resident.rs` track reuse |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Separate handwritten CPU formula implementation | Could feel like faster CPU path | Creates drift, duplicates maintenance, conflicts with CubeCL-only requirement | Stick with CubeCL CPU runtime even if warm-up costs exist, with caching/bridging to tame overhead |
| Embedding upstream libxc evaluator inside the production runtime | Provides ultimate oracle fidelity | Prevents safe Rust execution, dependency on C runtime, and defeats goal of Rust-native capture | Keep libxc confined to verification tooling; runtime remains CubeCL-based |
| Exposing C ABI structs as primary API surface | Familiar to C users, fewer wrappers | Hinders Rust ergonomics, leaks unsafe pointers, and makes validation harder | Provide compat layer for raw handles but promote ergonomic typed APIs as primary surface |

## Feature Dependencies

```
[Generated libxc inventory]
    └──requires──> [xtask parsing/codegen pipeline]
                       └──requires──> [libxc headers + csv artifacts]
[Layered API surface]
    └──requires──> [Safe core validation + generated metadata]
[Resident execution policies]
    └──requires──> [CubeCL unified compute]
[Verification harness] --enhances--> [System confidence in CubeCL parity]
[Separate CPU/GPU kernels] --conflicts--> [CubeCL unified compute]
```

### Dependency Notes

- **Generated libxc inventory requires the xtask parsing/codegen pipeline:** Coverage counts (85 functions, 649 IDs, 52 removed) depend on build-time artifacts described in `docs/libxc_rs_detailed_design.md` sections 3 and 19.
- **Layered API surface requires safe core validation + generated metadata:** Compatibility, core, and ergonomic entry points all leverage the generated `FunctionalMeta`, family/kind enums, and layout validators.
- **Resident execution policies require unified CubeCL compute:** Resident buffers, dirty-range uploads, and caching assume CubeCL kernels handle the actual evaluation.
- **Verification harness enhances confidence in CubeCL parity:** Running libxc oracle comparisons (abs/rel/ULP, CPU/GPU parity) justifies trusting the CubeCL-only runtime.
- **Separate CPU/GPU kernels conflicts with unified compute goal:** Maintaining diverging code would violate the CubeCL-only mandate and increase verification burden.

## MVP Definition

### Launch With (v1)

Minimum viable product — what's needed to validate the concept.

- [ ] Generated coverage of the full libxc inventory (functions, IDs, metadata) to make the API credible.
- [ ] Typed ergonomics (builder, validation, batch entry points) on top of the compatibility layer so Rust users can run evaluations safely.
- [ ] CubeCL-backed execution for CPU + GPU evaluation paths ensuring correctness without separate handwritten kernels.

### Add After Validation (v1.x)

Features to add once core is working.

- [ ] Resident execution buffers + output masking to optimize SCF-style loops once correctness is proven.
- [ ] Verification harness reporting abs/rel/ULP plus CPU/GPU parity to certify the runtime.

### Future Consideration (v2+)

Features to defer until product-market fit is established.

- [ ] Expanded autotuning/policy controls (precision heuristics, stream placement) once dispatch stability is confirmed.
- [ ] LCA/OEP or future functional families if new upstream inventory entries appear.

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Generated libxc inventory + metadata | HIGH | HIGH | P1 |
| Unified CubeCL execution for CPU/GPU | HIGH | HIGH | P1 |
| Typed builder/core ergonomic API | HIGH | MEDIUM | P1 |
| Resident execution & output masking | MEDIUM | MEDIUM | P2 |
| Verification harness & benchmarking | MEDIUM | HIGH | P2 |

**Priority key:**
- P1: Must have for launch
- P2: Should have, add when possible
- P3: Nice to have, future consideration

## Sources

- `.planning/PROJECT.md` (state, core value, requirements, constraints)
- `docs/libxc_rs_detailed_design.md` (public surface inventory, design principles, implementation plan)
- `README.md` (source tree outline reinforcing module responsibilities)

---
*Feature research for: Rust libxc re-architecture*  
*Researched: 2026-03-22*
