# Roadmap: libxc_rs

## Overview

This roadmap delivers a pure-Rust reimplementation of libxc 7.0.0 covering all 649 exchange-correlation functionals with f64 precision, CubeCL-based GPU compute, and C API compatibility. The journey follows the natural dependency chain: domain types and registry (foundation) -> math core and CubeCL validation (risk gate) -> I/O bundles and evaluation framework (interface contract) -> bulk kernel translation (core value) -> functional lifecycle and public API (usability) -> C compatibility layer (interop) -> GPU backends and performance (differentiator). Each phase delivers a verifiable capability that unblocks the next.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Foundation and Registry** - Domain types, error handling, static registry, dimension calculation, and oracle harness
- [ ] **Phase 2: Math Core and CubeCL Substrate** - All #[cube] math building blocks, CubeCL CPU backend, LDA_X canary kernel validation
- [ ] **Phase 3: Input/Output and Evaluation Framework** - I/O bundle types, output masks, dispatch routing, mixed functional accumulation
- [ ] **Phase 4: Bulk Kernel Translation** - All 270 maple2c kernel files translated to #[cube] functions with oracle verification
- [ ] **Phase 5: Functional Lifecycle and Hybrid Properties** - Functional struct, parameter management, hybrid queries, evaluation orchestration
- [ ] **Phase 6: Public API and C Compatibility** - Builder pattern, BatchEvaluator, ergonomic API, all 85 extern "C" functions
- [ ] **Phase 7: GPU Backends and Performance** - ROCM/HIP/WGPU backends, GPU buffer management, benchmarks, performance targets

## Phase Details

### Phase 1: Foundation and Registry
**Goal**: All domain types, error handling, static registry with 649 functionals, dimension calculation, and oracle verification harness are in place -- the project compiles, tests pass, and any functional can be looked up by ID or name with correct metadata
**Depends on**: Nothing (first phase)
**Requirements**: DOM-01, DOM-02, DOM-03, DOM-04, DOM-05, REG-01, REG-02, REG-03, REG-04, REG-05, ERR-01, ERR-02, ERR-03, VERIFY-01, BUILD-01, BUILD-02, BUILD-03, BUILD-04, BUILD-05
**Success Criteria** (what must be TRUE):
  1. Any of the 649 functional IDs can be looked up by numeric ID in O(1) or by name in O(log n), returning complete metadata (family, kind, references, ext_params)
  2. All 52 removed functional IDs return a typed error containing the replacement ID and name
  3. Dimension calculation returns correct array sizes for all family/spin/order combinations (verified against libxc util.c, including the 477-component case)
  4. The verify/ crate links against system libxc 7.0.0 via bindgen and can call C libxc functions to obtain oracle values
  5. cargo build, cargo test, and cargo clippy all pass with zero warnings
**Plans:** 3 plans

Plans:
- [x] 01-01-PLAN.md -- Domain types, error enum, and dimension calculation
- [x] 01-02-PLAN.md -- Xtask code generator, static registry, and FunctionalId wiring
- [x] 01-03-PLAN.md -- Oracle verification harness and build quality validation

### Phase 2: Math Core and CubeCL Substrate
**Goal**: All mathematical building blocks are implemented as #[cube] functions, validated against known values and libm references, and the CubeCL CPU backend produces bit-accurate f64 results for the LDA_X canary kernel
**Depends on**: Phase 1
**Requirements**: MATH-01, MATH-02, MATH-03, MATH-04, MATH-05, MATH-06, MATH-07, MATH-08, MATH-09, MATH-10, KERN-01, KERN-02
**Success Criteria** (what must be TRUE):
  1. safe_cbrt(-8.0) returns -2.0 (not NaN) on both CPU and GPU backends
  2. erf and erfc approximations match libm values to within f64 precision across the full input domain
  3. LDA_X kernel (both unpolarized and polarized) produces energy with relative error <= 10^-12 vs libxc oracle
  4. All #[cube] math functions produce identical results on CubeCL CPU backend as on native Rust (cross-backend consistency)
  5. Kernel launch wrapper correctly handles buffer creation, CubeCount/CubeDim calculation, and backend selection
**Plans**: 5 plans

Plans:
- [x] 02-01-PLAN.md — Math core: constants, powers, piecewise, polynomials, erf, spin, DFT quantities
- [x] 02-02-PLAN.md — Kernel launch infrastructure: backend selection, buffer management, CubeCount/CubeDim
- [x] 02-03-PLAN.md — LDA_X canary kernel translation and oracle verification
- [x] 02-04-PLAN.md — Math integration tests and workspace build quality gate
- [x] 02-05-PLAN.md — LDA_X edge-case and stress testing (thresholds, alpha, extreme spins)

### Phase 3: Input/Output and Evaluation Framework
**Goal**: Type-safe I/O bundles validate buffer sizes, output masks control which derivatives are computed, and the dispatch/accumulation framework correctly routes evaluation for single and mixed functionals
**Depends on**: Phase 2
**Requirements**: IO-01, IO-02, IO-03, IO-04, IO-05, EVAL-01, EVAL-02, EVAL-03, EVAL-04, EVAL-05
**Success Criteria** (what must be TRUE):
  1. LdaInput/GgaInput/MggaInput reject buffers whose length does not match the Dimensions for the given spin mode
  2. Output bundles support Option-based NULL-pointer semantics -- passing None for a derivative level skips its computation
  3. OutputMask bitflags correctly select which derivative levels (exc/vxc/fxc/kxc/lxc) to compute
  4. Mixed functional evaluation accumulates weighted results from auxiliary functionals, matching mix_func.c behavior
  5. Non-mixed evaluation hot path performs zero heap allocations
**Plans**: 3 plans

Plans:
- [x] 03-01-PLAN.md -- Input/output bundle types with validation and OutputMask bitflags
- [x] 03-02-PLAN.md -- LDA dispatch layer bridging bundles to CubeCL kernels
- [x] 03-03-PLAN.md -- Mixed functional workspace and accumulation framework

### Phase 4: Bulk Kernel Translation
**Goal**: All 270 maple2c kernel files are translated to Rust #[cube] functions preserving exact floating-point operation order, and every functional passes oracle verification through all applicable derivative orders and spin modes
**Depends on**: Phase 3
**Requirements**: KERN-03, KERN-04, KERN-05, KERN-06, KERN-07, KERN-08, KERN-09, VERIFY-02, VERIFY-03, VERIFY-04, VERIFY-05, VERIFY-06, VERIFY-07
**Success Criteria** (what must be TRUE):
  1. All ~43 LDA functional kernels pass oracle verification (energy relative error <= 10^-12)
  2. All ~130 GGA functional kernels pass oracle verification through applicable derivative orders
  3. All ~80 MGGA functional kernels pass oracle verification through applicable derivative orders
  4. Density thresholding correctly skips grid points below threshold and clamps spin densities
  5. Output accumulation uses += semantics to support mixed functional evaluation
**Plans**: 3 plans

Plans:
- [ ] 04-01: TBD
- [ ] 04-02: TBD
- [ ] 04-03: TBD
- [ ] 04-04: TBD
- [ ] 04-05: TBD

### Phase 5: Functional Lifecycle and Hybrid Properties
**Goal**: Users can construct a Functional instance by ID, configure external parameters and thresholds, query hybrid properties, and evaluate any of the 649 functionals through the Functional struct
**Depends on**: Phase 4
**Requirements**: FUNC-01, FUNC-02, FUNC-03, FUNC-04, FUNC-05, FUNC-06, HYB-01, HYB-02, HYB-03, HYB-04
**Success Criteria** (what must be TRUE):
  1. Functional::new(id, spin) returns a fully initialized instance with correct metadata, dimensions, thresholds, and default ext_params
  2. External parameters can be set/get by name or index, and modifying ext_params triggers recomputation of derived parameters
  3. Hybrid functionals correctly report their HybridType, CAM coefficients (omega, alpha, beta), and NLC coefficients (b, C)
  4. Auxiliary functionals for mixed/hybrid functionals are recursively constructed and iterable
  5. Drop implementation cleans up all resources without leaks
**Plans**: 3 plans

Plans:
- [ ] 05-01: TBD
- [ ] 05-02: TBD
- [ ] 05-03: TBD

### Phase 6: Public API and C Compatibility
**Goal**: The library provides an ergonomic Rust API with builder pattern and batch evaluation, plus a complete C compatibility layer that enables drop-in replacement for libxc in C/Fortran DFT codes
**Depends on**: Phase 5
**Requirements**: API-01, API-02, API-03, COMPAT-01, COMPAT-02, COMPAT-03
**Success Criteria** (what must be TRUE):
  1. FunctionalBuilder supports chained configuration of spin, thresholds, and ext_params with validation at build time
  2. BatchEvaluator reuses workspace across repeated evaluations without per-call allocation
  3. All 85 public C API functions are implemented as extern "C" with correct signatures matching libxc headers
  4. C-compatible struct layouts pass size/alignment assertions matching libxc's xc_func_type
  5. All unsafe code is confined to compat/, kernel/launch.rs, and GPU buffer management modules
**Plans**: 3 plans

Plans:
- [ ] 06-01: TBD
- [ ] 06-02: TBD
- [ ] 06-03: TBD

### Phase 7: GPU Backends and Performance
**Goal**: GPU evaluation delivers >5x CPU throughput on ROCM, all performance targets are met, and the benchmark suite provides regression detection
**Depends on**: Phase 6
**Requirements**: GPU-01, GPU-02, GPU-03, GPU-04, GPU-05, GPU-06, GPU-07, VERIFY-08, PERF-01, PERF-02, PERF-03, PERF-04, PERF-05
**Success Criteria** (what must be TRUE):
  1. ROCM backend produces results matching CPU to within 10^-14 relative error for all tested functionals
  2. GPU batch evaluation (100k points) achieves >5x CPU batch throughput on ROCM
  3. CPU batch evaluation (1000 points) is within 1.5x of libxc C performance
  4. WGPU backend returns a typed error at runtime if the device lacks f64 support (no silent f32 fallback)
  5. Benchmark suite with criterion detects performance regressions across key functionals
**Plans**: 3 plans

Plans:
- [ ] 07-01: TBD
- [ ] 07-02: TBD
- [ ] 07-03: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Foundation and Registry | 0/3 | Not started | - |
| 2. Math Core and CubeCL Substrate | 0/5 | Not started | - |
| 3. Input/Output and Evaluation Framework | 0/3 | Not started | - |
| 4. Bulk Kernel Translation | 0/5 | Not started | - |
| 5. Functional Lifecycle and Hybrid Properties | 0/3 | Not started | - |
| 6. Public API and C Compatibility | 0/3 | Not started | - |
| 7. GPU Backends and Performance | 0/3 | Not started | - |
