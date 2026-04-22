<<<<<<< HEAD
# Feature Landscape

**Domain:** Exchange-correlation functional library for DFT (libxc replacement)
**Researched:** 2026-04-09

## Competitive Landscape

The XC functional library space has a clear hierarchy:

| Library | Language | Functionals | Derivatives | GPU | Status |
|---------|----------|-------------|-------------|-----|--------|
| **libxc** | C/Fortran | 649 | Up to 4th | Experimental CUDA | De facto standard, 50+ codes |
| **XCFun** | C++ | ~40 | Arbitrary (AD) | No | Niche, last release 2020 |
| **ExchCXX** | C++ | 13 native | 1st-2nd | CUDA/HIP/SYCL | Wrapper around libxc + GPU kernels |
| **xcauto** | Python/JAX | ~5 | Arbitrary (AD) | Via JAX | Experimental/research |
| **jax_xc** | Python/JAX | ~400 (auto-translated) | Arbitrary (AD) | Via JAX | Research, not production |
| **Skala** | Python/PyTorch | 1 (ML) | Via AD | Via PyTorch | ML functional, Microsoft Research |
| **libnxc** | C/Python | ML models | 1st | CUDA | ML functional interface |

libxc_rs targets the libxc position: comprehensive, production-grade, used by DFT codes as their XC backend. The differentiators are safety, GPU native, and Rust ecosystem benefits.

## Table Stakes

Features users expect. Missing any of these means DFT codes cannot adopt the library.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **All 649 functional IDs** | Codes depend on specific functionals by ID; incomplete coverage = unusable | Very High | 270 maple2c kernel translations; largest single effort |
| **LDA/GGA/MGGA family support** | These are the three production functional families | High | Each has different input/output variable sets |
| **Derivatives through 4th order** | libxc 7.0 supports exc/vxc/fxc/kxc/lxc; response theory and TDDFT need 2nd+, some methods need 3rd/4th | High | Output dimension combinatorics up to 477 components for polarized 4th-order MGGA |
| **Unpolarized and polarized spin** | All DFT codes need both closed-shell and open-shell support | Medium | Doubles the dispatch paths; dimension rules differ |
| **Batch evaluation (np grid points)** | DFT codes evaluate XC on grids of thousands to millions of points | Medium | Core API pattern: pass array of np points, get back np results |
| **C API compatibility layer** | 50+ codes link against libxc's C API; drop-in replacement requires it | Medium | 85 extern "C" functions mapping to internal Rust |
| **Hybrid functional properties** | Global hybrids (B3LYP, PBE0) and range-separated (CAM-B3LYP, wB97X) are heavily used | Medium | Library reports coefficients/ranges; calling code handles exact exchange |
| **External parameters** | Many functionals have tunable parameters (e.g., omega in range-separated) | Medium | Per-functional parameter arrays with names, descriptions, defaults |
| **Density thresholding** | Numerical stability requires skipping near-zero density regions | Low | Threshold per density/sigma/tau; grid points below threshold zeroed |
| **Functional metadata/introspection** | Codes need to query family, kind, flags, references, parameter info | Low | Static registry data; no computation |
| **Mixed functional accumulation** | Composite functionals (e.g., B3LYP = VWN + B88 + LYP + HF) accumulate weighted component outputs | Medium | += semantics on output arrays with mix coefficients |
| **NULL output pointer semantics** | Codes only want specific derivatives; skip others via NULL pointers | Low | Maps to Option<&mut [f64]> in Rust |
| **Removed functional ID handling** | Legacy codes may reference old IDs; must report replacement | Low | 52 removed IDs with aliases |
| **f64 precision throughout** | Computational chemistry demands double precision; no silent degradation | Low | Policy decision, not implementation effort |

## Differentiators

Features that set libxc_rs apart from libxc. Not expected by users today, but deliver competitive advantage.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Native GPU execution (CUDA/HIP/WGPU)** | libxc has only experimental CUDA; ExchCXX covers 13 functionals. GPU-native evaluation of ALL 649 functionals is unprecedented | Very High | CubeCL unified kernel approach; single source for CPU+GPU |
| **Memory safety (Rust)** | libxc has had buffer overflow bugs; Rust eliminates entire class of memory errors | Low | Inherent to language choice; no extra implementation cost |
| **Type-safe API** | libxc uses int flags and void pointers; Rust enums/newtypes catch errors at compile time | Medium | Three-layer API: C-compat, typed core, ergonomic high-level |
| **Zero-cost static registry** | libxc does runtime registration; Rust can encode all 649 functionals as compile-time constants | Medium | &'static data, no heap allocation at init |
| **No C/Fortran build dependency** | libxc requires C compiler + autotools/cmake; Rust builds with cargo | Low | Huge DX win for Rust-native DFT codes |
| **Thread safety by design** | libxc's xc_func_type is mutable and not thread-safe; Rust ownership model prevents data races | Medium | Functional instances can be safely shared across threads |
| **Typed error handling** | libxc returns int error codes; Rust Result types with descriptive errors | Low | thiserror v2 at library boundary |
| **Builder pattern / ergonomic API** | libxc requires alloc-init-set_params-evaluate-end-free lifecycle | Low | Rust builder pattern, BatchEvaluator for common workflows |
| **Output mask bitflags** | libxc has 36+ C-style method variants for derivative combinations; single method + mask is cleaner | Low | OutputMask bitflags replacing explosion of API functions |
| **GPU-resident buffer management** | Minimize host-device transfers for repeated evaluations on same grid | High | Keep density/output on device across SCF iterations |
| **Branch-free piecewise evaluation** | GPU thread divergence from if/else in piecewise functions hurts performance | Medium | Branch-free piecewise3/5 implementations |

## Anti-Features

Features to explicitly NOT build. These represent scope traps, maintenance burdens, or architectural dead ends.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| **Automatic differentiation engine** | XCFun shows AD can be 1000x slower than hand-coded derivatives; catastrophic error cancellation in Taylor series; would require re-architecting all 649 functionals | Translate libxc's maple2c hand-derived kernels directly; they are correct and fast |
| **VV10 non-local correlation kernel** | Fundamentally different evaluation strategy (double integration over grid); cannot fit into local XC evaluation API | Document as future work; return typed error when VV10 flag is set |
| **1D/2D dimensionality modes** | Niche use case (quantum wires, 2D materials with specialized codes); libxc marks as niche | Support 3D only; return error for 1D/2D flags |
| **LCA/OEP functional families** | Deprecated/internal in libxc; no production DFT codes depend on these | Exclude from registry; document as intentionally omitted |
| **f32 evaluation mode** | Computational chemistry precision requirements mandate f64; mixed precision introduces subtle numerical errors | f64 only; typed error if GPU device lacks f64 support |
| **Runtime code generation from maple2c** | Adds build-time complexity, external tool dependency, fragility | Pre-translated static Rust kernels only |
| **Async/streaming evaluation API** | DFT codes use synchronous batch evaluation; async adds complexity with no consumer demand | Synchronous batch API; GPU operations are implicitly async underneath |
| **Python bindings in initial release** | Scope explosion; PyO3 bindings are straightforward to add later | Provide C API layer first; Python bindings as separate crate later |
| **Maple/symbolic math integration** | libxc uses Maple for code generation; coupling to proprietary CAS is a maintenance burden | One-time manual translation; verify against oracle |
| **Machine-learned functional support** | Skala/libnxc show ML functionals are a separate concern with different evaluation patterns (neural network inference vs. analytical formulas) | Out of scope; ML functionals can use this library's infrastructure as a starting point |
| **Fortran bindings in initial release** | Fortran interop through C API is standard practice; direct Fortran module adds complexity | Fortran codes call through C API compatibility layer |
=======
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
>>>>>>> origin/main

## Feature Dependencies

```
<<<<<<< HEAD
Static Registry (functional metadata)
  |-> Functional Evaluation (needs registry for dispatch)
  |-> Introspection API (queries registry)
  |-> External Parameters (stored per functional in registry)

Domain Model (enums, newtypes)
  |-> Input/Output Bundles (use Family, Spin, DerivativeOrder types)
  |-> Dimension Calculation (depends on Family, Spin, DerivativeOrder)
  |-> Evaluation Dispatch (pattern matches on Family)

Mathematical Core (shared #[cube] building blocks)
  |-> All 270 Kernel Translations (use pow_1_3, safe_cbrt, erf, etc.)
  |-> Branch-free Piecewise (used by kernel translations)

CubeCL Substrate (runtime, device selection)
  |-> Kernel Execution (compiles and launches kernels)
  |-> GPU Buffer Management (device memory allocation)
  |-> Backend Selection (feature-gated CUDA/HIP/WGPU)

Input/Output Bundles (buffer validation, dimension calc)
  |-> Evaluation Entry Points (accept bundles, dispatch to kernels)
  |-> NULL Pointer Semantics (Option<&mut [f64]> in output bundles)

Kernel Translations (all 270 maple2c -> Rust #[cube])
  |-> Functional Evaluation (core computation)
  |-> Mixed Functional Accumulation (weighted sum of component kernels)

Density Thresholding
  |-> Evaluation Loop (applied before kernel invocation)

Hybrid Properties (coefficients, ranges)
  |-> Hybrid Functional Reporting (codes query for exact exchange setup)
  |-> Auxiliary Functionals (hybrid components reference base functionals)

C API Compatibility Layer
  |-> Requires: all above features working internally
  |-> Drop-in replacement for libxc in C/Fortran codes

Ergonomic High-Level API (Builder, BatchEvaluator)
  |-> Requires: typed core API working
  |-> Nice-to-have layer on top of core
```

## MVP Recommendation

### Phase 1: Foundation (must complete first)
1. **Domain model** -- enums, newtypes, dimension calculation
2. **Static registry** -- all 649 functional metadata entries
3. **Input/output bundles** -- buffer validation, dimension rules

### Phase 2: Core Evaluation
4. **Mathematical core** -- shared numerical building blocks as #[cube] functions
5. **CubeCL substrate** -- CPU backend working, kernel compilation pipeline
6. **Kernel translations** -- start with most-used functionals (LDA: VWN5, PW; GGA: PBE, B88, LYP; MGGA: TPSS, SCAN)

### Phase 3: Complete Coverage
7. **All 270 kernel translations** -- bulk translation effort
8. **Mixed functional accumulation** -- composite functionals (B3LYP, PBE0, etc.)
9. **Hybrid properties** -- report coefficients for exact exchange

### Phase 4: Integration Layer
10. **C API compatibility** -- extern "C" functions for drop-in replacement
11. **Ergonomic Rust API** -- builder pattern, BatchEvaluator
12. **GPU backends** -- CUDA/HIP/WGPU feature-gated support

### Defer
- **Python/Fortran bindings**: Add after core is stable; separate crates
- **VV10 non-local**: Fundamentally different evaluation model; future work
- **GPU-resident buffer management**: Optimization after correctness is proven

## Sources

- [libxc official site](https://libxc.gitlab.io/) -- 50+ integrated codes, functional listings
- [libxc 7.0.0 release](https://gitlab.com/libxc/libxc/-/releases/7.0.0) -- 23 new functionals, API changes
- [XCFun GitHub](https://github.com/dftlibs/xcfun) -- arbitrary-order AD, ~40 functionals
- [ExchCXX GitHub](https://github.com/wavefunction91/ExchCXX) -- 13 GPU-native functionals, CUDA/HIP/SYCL
- [xcauto GitHub](https://github.com/dftlibs/xcauto) -- JAX-based AD for XC derivatives
- [Microsoft Skala](https://github.com/microsoft/skala) -- ML exchange-correlation functional
- [libnxc GitHub](https://github.com/semodi/libnxc) -- ML functional interface mimicking libxc API
- [libxc source code](libxc-master/src/xc.h) -- direct inspection of 85 public C API functions
- [libxc_rs design document](docs/design/libxc_rs_detailed_design.md) -- implementation-ready architecture
=======
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
>>>>>>> origin/main
