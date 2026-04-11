# libxc_rs

## What This Is

<<<<<<< HEAD
libxc_rs is a from-scratch Rust reimplementation of the libxc 7.0.0 exchange-correlation (XC) functional library used in density functional theory (DFT) calculations. It covers all 649 functionals across LDA, GGA, MGGA, and hybrid families, with derivatives through 4th order, polarized and unpolarized spin modes, and a unified CubeCL compute substrate that serves both CPU and GPU execution from a single kernel source. The library targets computational chemists and DFT code developers who need a safe, performant, GPU-capable XC functional library without C/Fortran FFI dependencies.

## Core Value

Numerically accurate (energy relative error <= 10^-12 vs libxc oracle) evaluation of all 649 exchange-correlation functionals from a single pure-Rust codebase that runs on both CPU and GPU without code duplication.
=======
`libxc_rs` is a Rust re-architecture of the full public `libxc` API surface for the `libxc 7.0.0` inventory captured in `docs/libxc_rs_detailed_design.md`. It preserves reachability to libxc public capabilities, but replaces the original C-style surface with a layered Rust API: a low-level compatibility layer, a typed safe core, and an ergonomic high-level interface for host and device execution.

This project is explicitly designed around a unified CubeCL compute path. All numerical evaluation on CPU and GPU goes through CubeCL kernels, while libxc itself remains an oracle used only by verification tooling.

## Core Value

Deliver full libxc public capability coverage through a safer Rust API without splitting CPU and GPU semantics into separate evaluator implementations.
>>>>>>> origin/main

## Requirements

### Validated

<<<<<<< HEAD
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

### Active

- [ ] Pure Rust implementation -- no runtime C/Fortran FFI dependency in the production path
- [ ] Three-layer API: compatibility layer (C API 1:1 mapping), typed safe core, ergonomic high-level interface
- [ ] All 52 removed functional IDs return typed error with replacement ID
- [ ] All 85 public C API functions mapped to Rust equivalents
- [ ] All 270 maple2c kernel files translated to Rust #[cube] functions preserving f64 precision and operation order
- [ ] Density thresholding: grid points below threshold skipped, spin densities clamped
- [ ] Output accumulation via += for mixed functional support
- [ ] Unified CubeCL substrate: single kernel source compiles to cubecl-cpu (always), cubecl-cuda, cubecl-hip, cubecl-wgpu (feature-gated)
- [ ] GPU-resident buffer management minimizing host-device transfers
- [ ] f64-only precision policy -- no silent f32 degradation
- [ ] Evaluation orchestration: dispatch by family/order/spin, mixed functional accumulation with workspace
- [ ] Functional instance lifecycle: construction, ext_params, thresholds, auxiliary functionals for hybrids
- [ ] Builder pattern and BatchEvaluator for ergonomic API
- [ ] C compatibility layer: extern "C" functions for all 85 public C API functions
- [ ] Hybrid functional properties: HybridType, CAM coefficients, NLC coefficients, auxiliary functionals
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
=======
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
>>>>>>> origin/main

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
<<<<<<< HEAD
| CubeCL unified kernels over separate CPU/GPU implementations | Eliminates 649-functional duplication; prevents CPU/GPU numerical drift | -- Pending |
| Static registry in Rust source over runtime registration | Zero allocation at startup; compiler verifies completeness; no I/O during init | -- Pending |
| Three-layer API architecture | Serves C/Fortran integrators, pure Rust users, and application developers without compromising any | -- Pending |
| Branch-free piecewise3/5 over if/else | Avoids GPU thread divergence and CPU branch misprediction | -- Pending |
| thiserror v2 at library boundary, anyhow in tools | Standard Rust error pattern; typed errors for consumers, ergonomic for internal tools | -- Pending |
| SoA interleaved buffer layout matching libxc | Enables bit-exact oracle comparison; reasonable cache locality | -- Pending |
| Single Functional struct over Generic<F: Family> | Most users want runtime family selection; generic over-constrains API | -- Pending |
| OutputMask bitflags over per-derivative methods | Avoids 36 C-style method variants; clean API | -- Pending |
=======
| Use CubeCL for all numerical execution, including CPU | One kernel codebase reduces CPU/GPU drift and concentrates optimization effort | — Pending |
| Keep libxc out of the production runtime and use it only in verification tooling | Preserves a Rust-native runtime while still grounding correctness in the upstream oracle | — Pending |
| Structure the public surface in three layers: compat, safe core, ergonomic high-level API | Preserves complete reachability while giving Rust callers typed, safer abstractions | — Pending |
| Generate metadata, registry, removed-ID, and dispatch artifacts at build time | Inventory completeness and constant-time lookups depend on generated source artifacts rather than runtime parsing | — Pending |
| Treat resident execution and selective output materialization as first-class capabilities | GPU throughput and repeated CPU/GPU workloads depend on transfer minimization and output masking | — Pending |
>>>>>>> origin/main

## Evolution

This document evolves at phase transitions and milestone boundaries.

<<<<<<< HEAD
**After each phase transition** (via `/gsd-transition`):
=======
**After each phase transition** (via `$gsd-transition`):
>>>>>>> origin/main
1. Requirements invalidated? -> Move to Out of Scope with reason
2. Requirements validated? -> Move to Validated with phase reference
3. New requirements emerged? -> Add to Active
4. Decisions to log? -> Add to Key Decisions
5. "What This Is" still accurate? -> Update if drifted

<<<<<<< HEAD
**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check -- still the right priority?
3. Audit Out of Scope -- reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-09 after Phase 1 completion*
=======
**After each milestone** (via `$gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check -> still the right priority?
3. Audit Out of Scope -> reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-03-22 after initialization*
>>>>>>> origin/main
