# libxc_rs

## What This Is

libxc_rs is a from-scratch Rust reimplementation of the libxc 7.0.0 exchange-correlation (XC) functional library used in density functional theory (DFT) calculations. It covers all 649 functionals across LDA, GGA, MGGA, and hybrid families, with derivatives through 4th order, polarized and unpolarized spin modes, and a unified CubeCL compute substrate that serves both CPU and GPU execution from a single kernel source. The library targets computational chemists and DFT code developers who need a safe, performant, GPU-capable XC functional library without C/Fortran FFI dependencies.

## Core Value

Numerically accurate (energy relative error <= 10^-12 vs libxc oracle) evaluation of all 649 exchange-correlation functionals from a single pure-Rust codebase that runs on both CPU and GPU without code duplication.

## Requirements

### Validated

- [x] Domain model: Family, Kind, Spin, DerivativeOrder, FunctionalId, FunctionalFlags enums/newtypes — Validated in Phase 1
- [x] All 649 functional IDs present in static registry with zero runtime registration cost — Validated in Phase 1
- [x] Static FunctionalMeta with references, ext_params, hybrid terms as `&'static` data — Validated in Phase 1
- [x] O(1) registry lookup by ID, O(log n) by name — Validated in Phase 1
- [x] Dimension calculation matching libxc's util.c rules (up to 477 components for 4th-order polarized MGGA) — Validated in Phase 1
- [x] Oracle verification harness in verify/ crate using bindgen against system libxc 7.0.0 — Validated in Phase 1
- [x] Input bundles (LdaInput, GgaInput, MggaInput) with buffer size validation — Validated in Phase 3
- [x] Output bundles with Option<&mut [f64]> NULL-pointer semantics and OutputMask bitflags — Validated in Phase 3
- [x] Mathematical core: shared #[cube] numerical building blocks (pow_1_3, safe_cbrt, piecewise3/5, erf/erfc, Horner evaluation, DFT quantities) — Validated in Phase 2
- [x] Evaluation orchestration: dispatch by family/order/spin, mixed functional accumulation with workspace — Validated in Phase 3
- [x] Builder pattern and BatchEvaluator for ergonomic API (sealed EvaluateInput auto-dispatch by family) — Validated in Phase 6
- [x] All 85 public C API functions mapped to Rust equivalents (87 extern "C" symbols, ≥85) — Validated in Phase 6
- [x] C compatibility layer: extern "C" functions + opaque #[repr(C)] handles + hand-written include/xc.h (gcc c89/c99 clean) — Validated in Phase 6
- [x] Hybrid functional properties: HybridType, CAM coefficients, NLC coefficients, auxiliary functionals — Validated in Phase 6

### Active

- [ ] Pure Rust implementation -- no runtime C/Fortran FFI dependency in the production path
- [ ] Three-layer API: compatibility layer (C API 1:1 mapping), typed safe core, ergonomic high-level interface
- [ ] All 52 removed functional IDs return typed error with replacement ID
- [ ] All 270 maple2c kernel files translated to Rust #[cube] functions preserving f64 precision and operation order
- [ ] Density thresholding: grid points below threshold skipped, spin densities clamped
- [ ] Output accumulation via += for mixed functional support
- [ ] Unified CubeCL substrate: single kernel source compiles to cubecl-cpu (always), cubecl-cuda, cubecl-hip, cubecl-wgpu (feature-gated)
- [ ] GPU-resident buffer management minimizing host-device transfers
- [ ] f64-only precision policy -- no silent f32 degradation
- [ ] Evaluation orchestration: dispatch by family/order/spin, mixed functional accumulation with workspace
- [ ] Functional instance lifecycle: construction, ext_params, thresholds, auxiliary functionals for hybrids
- [ ] Error types with thiserror v2 at library boundary, anyhow in verify/benches/xtask
- [ ] Performance: CPU batch within 1.5x of libxc C, GPU batch >5x CPU throughput, zero heap allocation in non-mixed hot path
- [ ] Benchmark suite with criterion for regression detection

### Out of Scope

- VV10 non-local correlation kernel implementation -- requires fundamentally different (non-local) evaluation strategy; track as future work
- 1D/2D dimensionality evaluation -- 3D is the primary use case; libxc marks these as niche
- LCA/OEP functional families -- libxc marks as deprecated/internal
- f32 evaluation mode -- precision requirements mandate f64 throughout
- Runtime C header parsing or code generation from maple2c -- pure Rust static data only
- Async/streaming evaluation API -- synchronous batch API sufficient for DFT integration

## Context

- libxc 7.0.0 is the correctness oracle; vendored source at `libxc-master/`
- 270 maple2c auto-generated C kernel files across 6 directories are the translation source
- 10,312 regression tests with 4 test systems (H, Li, BrOH, BrOH+) provide verification data
- CubeCL 0.9.0 is the compute substrate; docs at `docs/manual/Cubecl/`
- The design document is implementation-ready with complete API mappings, data structures, module decomposition, and processing flows
- Phase 1 (infrastructure/types) is marked COMPLETE in the design doc; Phase 2 (I/O bundles, CubeCL substrate) is partially complete
- Key unresolved items: CubeCL erf/erfc intrinsic availability, cbrt negative handling, max kernel size before compilation degrades, WGPU f64 support, CubeCL ComputeClient thread safety

## Constraints

- **Tech stack**: Pure Rust + CubeCL 0.9.0; no C/Fortran in production path
- **Precision**: f64 only; energy relative error <= 10^-12 vs libxc oracle
- **Dependencies**: cubecl 0.9.0, thiserror 2.0, bitflags 2.10, bytemuck 1.25 (production); bindgen, anyhow, criterion, rayon (verification/benchmark only)
- **Compatibility**: Must provide extern "C" layer for drop-in replacement in C/Fortran DFT codes
- **Operation order**: Maple2c formula translations must preserve floating-point operation order for bit-level equivalence
- **GPU precision**: No silent f32 fallback; typed error if device lacks f64 support

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| CubeCL unified kernels over separate CPU/GPU implementations | Eliminates 649-functional duplication; prevents CPU/GPU numerical drift | -- Pending |
| Static registry in Rust source over runtime registration | Zero allocation at startup; compiler verifies completeness; no I/O during init | -- Pending |
| Three-layer API architecture | Serves C/Fortran integrators, pure Rust users, and application developers without compromising any | -- Pending |
| Branch-free piecewise3/5 over if/else | Avoids GPU thread divergence and CPU branch misprediction | -- Pending |
| thiserror v2 at library boundary, anyhow in tools | Standard Rust error pattern; typed errors for consumers, ergonomic for internal tools | -- Pending |
| SoA interleaved buffer layout matching libxc | Enables bit-exact oracle comparison; reasonable cache locality | -- Pending |
| Single Functional struct over Generic<F: Family> | Most users want runtime family selection; generic over-constrains API | -- Pending |
| OutputMask bitflags over per-derivative methods | Avoids 36 C-style method variants; clean API | -- Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? -> Move to Out of Scope with reason
2. Requirements validated? -> Move to Validated with phase reference
3. New requirements emerged? -> Add to Active
4. Decisions to log? -> Add to Key Decisions
5. "What This Is" still accurate? -> Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check -- still the right priority?
3. Audit Out of Scope -- reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-05-25 after Phase 6 (Public API & C Compatibility) completion*
