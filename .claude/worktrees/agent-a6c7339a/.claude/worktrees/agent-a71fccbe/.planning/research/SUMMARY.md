# Project Research Summary

**Project:** libxc_rs -- Pure Rust DFT XC Functional Library with GPU Compute
**Domain:** Scientific computing / computational chemistry (exchange-correlation functional library)
**Researched:** 2026-04-09
**Confidence:** MEDIUM-HIGH

## Executive Summary

libxc_rs is a ground-up Rust reimplementation of libxc, the de facto standard exchange-correlation (XC) functional library used by 50+ density functional theory (DFT) codes. The project must reproduce 649 functionals with bit-level accuracy (10^-12 relative error) through 4th-order derivatives, while adding GPU-native evaluation via CubeCL. The competitive landscape shows no existing library covers all 649 functionals on GPU -- ExchCXX handles only 13, making comprehensive GPU coverage an unprecedented differentiator.

The recommended approach is to build on CubeCL 0.9.0 as the unified compute substrate, writing all numerical kernels as `#[cube]` functions that compile to CPU SIMD, CUDA PTX, and HIP from a single source. The architecture mirrors libxc's proven four-layer structure (kernel -> work template -> dispatch -> API) translated into idiomatic Rust with static registry, type-safe I/O bundles, and feature-gated backends. The critical path is validating CubeCL's ability to produce bit-accurate f64 results on a canary kernel (LDA exchange) before committing to translating 270 kernel files containing ~4 million lines of auto-generated C math.

The top risks are: (1) floating-point operation order divergence during kernel translation silently corrupting higher-order derivatives, (2) CubeCL macro limitations breaking kernel compilation in non-obvious ways, (3) enormous MGGA kernels (up to 100K lines) exceeding GPU compiler limits, and (4) the WGPU backend fundamentally lacking f64 support. Mitigations are well-understood: mechanical translation preserving exact operation order, comprehensive math helper validation before kernel work, early profiling of the largest kernels, and strict f64 capability gating at runtime.

## Key Findings

### Recommended Stack

CubeCL 0.9.0 is the cornerstone technology -- the only Rust crate providing single-source GPU/CPU kernels. CPU backend is always available for CI and fallback; CUDA is the primary GPU target with full f64 support. The stack is deliberately minimal: bitflags for output masks, bytemuck for GPU buffer casting, thiserror for typed errors, and a verification harness using bindgen + the vendored libxc 7.0.0 C source as oracle.

**Core technologies:**
- **CubeCL 0.9.0**: Unified GPU/CPU kernel authoring -- only option for single-source Rust compute kernels across CUDA/HIP/CPU
- **bitflags 2.10.0**: OutputMask type -- zero-overhead type-safe bitfields, de facto standard
- **bytemuck 1.25.0**: GPU buffer casting -- required for CubeCL's `client.create(cast_slice())` pattern
- **thiserror 2.0.18**: Library error types -- standard for typed errors at public API boundary
- **bindgen 0.72.1 + vendored libxc**: Oracle verification -- FFI bindings to C libxc for regression testing

**Critical stack gaps:** CubeCL lacks `erf`/`erfc` and `cbrt` intrinsics. These must be implemented as pure `#[cube]` functions using rational approximations. This is feasible but adds Phase 2 work.

### Expected Features

**Must have (table stakes):**
- All 649 functional IDs with LDA/GGA/MGGA family support
- Derivatives through 4th order (exc/vxc/fxc/kxc/lxc)
- Unpolarized and polarized spin evaluation
- Batch evaluation over np grid points
- C API compatibility layer (85 extern "C" functions for drop-in replacement)
- Hybrid functional properties, external parameters, density thresholding
- f64 precision throughout with no silent degradation

**Should have (differentiators):**
- Native GPU execution across CUDA/HIP (unprecedented for all 649 functionals)
- Memory safety, type-safe API, zero-cost static registry
- Thread safety by design, typed error handling
- Branch-free piecewise evaluation for GPU performance
- GPU-resident buffer management for SCF iteration loops

**Defer (v2+):**
- Python/Fortran bindings (add as separate crates after core stabilizes)
- VV10 non-local correlation (fundamentally different evaluation model)
- GPU-resident buffer optimization (correctness first)
- WGPU backend for production use (f64 not supported in WebGPU spec)

**Anti-features (never build):**
- Automatic differentiation engine (1000x slower than hand-coded, per XCFun experience)
- f32 evaluation mode (unacceptable precision loss)
- 1D/2D dimensionality modes (niche, out of scope)

### Architecture Approach

The architecture follows libxc's proven four-layer pattern (kernel -> work template -> dispatch -> API) mapped to Rust modules with strict dependency ordering. Leaf modules (`math/`, `model/`) have zero dependencies. Kernels depend only on math. The work template pattern centralizes grid-point loops, thresholding, and clamping in per-family launch wrappers so that individual kernels contain only pure arithmetic. All 649 functionals share static metadata via a compile-time registry with O(1) ID lookup.

**Major components:**
1. **model/ + error/** -- Domain types (Family, Spin, FunctionalId) and typed errors. Foundation for everything.
2. **meta/ + registry/ + dims/** -- Static functional metadata (649 entries), lookup tables, dimension calculation. Pure data, no runtime behavior.
3. **math/** -- CubeCL `#[cube]` building blocks (pow_1_3, safe_cbrt, erf, piecewise3/5). Leaf module, most critical for correctness.
4. **kernel/** -- Per-functional CubeCL kernels (~270 files) plus family-specific launch wrappers. The bulk of the codebase.
5. **eval/** -- Dispatch routing by (family, order, spin), mixed/hybrid accumulation, workspace management.
6. **func/ + hybrid/** -- Functional lifecycle, parameter management, hybrid queries.
7. **gpu/ + api/ + compat/** -- GPU buffer management, ergonomic builder API, C FFI compatibility layer.

### Critical Pitfalls

1. **Floating-point operation order divergence** -- Translate maple2c formulas mechanically, preserving every temporary and exact parenthesization. Never simplify expressions. Build an automated translator rather than hand-translating 270 files. Test every derivative order against oracle.
2. **CubeCL `#[cube]` macro limitations** -- The macro understands a restricted Rust subset. Build and exhaustively test all math helpers (piecewise3/5, safe_cbrt, erf, pow_1_3) before any kernel translation. Use mutable-assignment pattern for conditionals, associated-function style for math.
3. **100K-line kernels exceeding GPU compiler limits** -- Separate each derivative order into its own kernel. Profile the 5 largest MGGA kernels early. Consider CPU-only fallback for truly enormous kernels.
4. **`cbrt` of negative numbers producing NaN on GPU** -- Implement `safe_cbrt` as `sign(x) * pow(abs(x), 1.0/3.0)`. This affects every polarized functional. Must be validated in Phase 2.
5. **WGPU f64 impossibility** -- Feature-gate WGPU, return typed error at runtime. CUDA is the primary GPU target. Remove `wgpu` from default features.

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Foundation and Infrastructure
**Rationale:** Everything depends on domain types, error handling, and the verification harness. The registry and dimension calculation are pure data that can be validated independently. Establishing the oracle harness early de-risks all subsequent phases.
**Delivers:** Domain model, error types, static registry (649 entries), dimension calculation, verified oracle harness (libxc 7.0.0 FFI).
**Addresses:** Functional metadata/introspection, removed functional ID handling, density thresholding defaults.
**Avoids:** Verify harness linking wrong libxc version (Pitfall 13), missing removed functionals (Pitfall 14).

### Phase 2: Math Core and CubeCL Substrate
**Rationale:** This is the make-or-break validation phase. If CubeCL cannot produce bit-accurate f64 results for a simple LDA kernel, the entire GPU strategy must be reconsidered. All math building blocks must be proven correct before the bulk translation effort.
**Delivers:** All `#[cube]` math helpers (pow_1_3, safe_cbrt, erf/erfc, piecewise3/5, constants), CubeCL CPU backend integration, kernel launch wrapper for LDA family, LDA_X canary kernel validated against oracle.
**Uses:** CubeCL 0.9.0 (cpu feature), bytemuck, approx (for testing).
**Avoids:** CubeCL macro limitations (Pitfall 2), cbrt NaN (Pitfall 4), WGPU f64 (Pitfall 5), threshold mismatch (Pitfall 6).

### Phase 3: Input/Output Bundles and Evaluation Framework
**Rationale:** I/O bundle types define the evaluation interface contract. Must be stable before bulk kernel translation. The dispatch and accumulation framework needs to be proven with a handful of representative functionals before scaling to 270.
**Delivers:** Validated input/output bundle types (LDA/GGA/MGGA), output mask bitflags, evaluation dispatch, mixed functional accumulation (mix.rs), workspace management.
**Addresses:** Batch evaluation, NULL output pointer semantics, mixed functional accumulation, output accumulation semantics.
**Avoids:** Accumulation += vs = (Pitfall 7).

### Phase 4: Bulk Kernel Translation
**Rationale:** The largest single effort (~270 files, ~4M lines of C). Depends on validated math core, proven CubeCL pipeline, and stable I/O types. Build an automated translator, validate incrementally: LDA first (~43), then GGA (~130), then MGGA (~75). Test the 5 largest MGGA kernels early to surface compiler limit issues.
**Delivers:** All 649 functional kernels translated and oracle-validated through all derivative orders.
**Addresses:** All 649 functional IDs, LDA/GGA/MGGA family support, derivatives through 4th order, unpolarized and polarized spin.
**Avoids:** Operation order divergence (Pitfall 1), translator fragility (Pitfall 8), constant precision loss (Pitfall 11), kernel size limits (Pitfall 3).

### Phase 5: Functional Lifecycle and API
**Rationale:** With all kernels working, build the public-facing Functional struct with full lifecycle management, parameter handling, and the ergonomic Rust API. The C compatibility layer goes here because it wraps the complete internal API.
**Delivers:** Functional struct with lifecycle management, external parameter support, hybrid functional queries, builder pattern API, BatchEvaluator, C API compatibility layer (85 extern "C" functions).
**Addresses:** External parameters, hybrid functional properties, C API compatibility, ergonomic API, type-safe API.
**Avoids:** External parameter mutation staleness (Pitfall 9).

### Phase 6: GPU Backends and Optimization
**Rationale:** GPU support is additive on top of a working CPU implementation. Feature-gate CUDA and HIP backends. Profile and optimize: branch-free piecewise, register pressure on large kernels, GPU-resident buffers. WGPU is best-effort only.
**Delivers:** CUDA backend (primary), HIP backend (secondary), GPU buffer management, performance benchmarks, tiered GPU support (LDA/GGA kernels first, large MGGA as feasible).
**Addresses:** Native GPU execution, GPU-resident buffer management, branch-free piecewise evaluation.
**Avoids:** Large kernel register pressure (Pitfall 3), thread divergence (Pitfall 10), missing inlining (Pitfall 12).

### Phase Ordering Rationale

- **Phases 1-2 are sequential and non-negotiable.** Phase 2's canary kernel is the project's key technical risk gate. If CubeCL cannot reproduce LDA_X exactly, the project pivots before investing in 270 kernel translations.
- **Phase 3 before Phase 4** because the I/O bundle types and evaluation framework must be stable before bulk translation. Changing the kernel function signature after translating 100 kernels would be catastrophic.
- **Phase 4 is the critical mass** -- it delivers the core value proposition (649 functionals). It should be the longest phase and can be parallelized across contributors (one per functional family).
- **Phase 5 after Phase 4** because the C API compatibility layer needs all functionals working internally. The API design is informed by which patterns emerge during bulk translation.
- **Phase 6 last** because GPU is a differentiator, not table stakes. A CPU-only library with all 649 functionals is already useful. GPU support is additive value.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 2:** CubeCL 0.9.0 `#[cube]` macro behavior needs runtime validation. The erf/erfc rational approximation precision must be verified. Research the exact CubeCL compilation pipeline for f64 kernels.
- **Phase 4:** The automated maple2c-to-Rust translator needs a pattern catalog of ALL maple2c idioms before design. Deorbitalized functionals and hybrid-specific patterns need special handling.
- **Phase 6:** GPU register pressure limits for large MGGA kernels. CubeCL HIP backend maturity. CUDA PTX instruction limits for 100K-line kernels.

Phases with standard patterns (skip research-phase):
- **Phase 1:** Standard Rust domain modeling and FFI patterns. Well-documented.
- **Phase 3:** I/O bundle validation and dispatch patterns are straightforward Rust. The architecture doc provides complete component design.
- **Phase 5:** Builder pattern, lifecycle management, and C FFI are well-established Rust patterns.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | MEDIUM-HIGH | CubeCL 0.9.0 confirmed on crates.io; f64 usage verified in vendored docs. WGPU f64 gap confirmed. erf/erfc/cbrt gaps identified with clear mitigations. HIP backend maturity is LOW confidence. |
| Features | HIGH | Direct analysis of libxc 7.0.0 source code and public API (85 functions, 649 functionals). Competitive landscape well-mapped. Feature dependencies clearly traced. |
| Architecture | HIGH | Based on direct source analysis of vendored libxc C code AND a detailed design document. Four-layer pattern is proven by libxc's 20+ year production history. Rust module structure is well-specified. |
| Pitfalls | HIGH | Pitfalls derived from direct C source analysis, known CubeCL limitations, IEEE 754 semantics, and GPU hardware constraints. Each pitfall has specific detection criteria and prevention strategies. |

**Overall confidence:** MEDIUM-HIGH

The core approach (CubeCL for unified compute, mechanical kernel translation, oracle-based verification) is well-founded. The main uncertainty is CubeCL 0.9.0's behavior at scale -- specifically whether the `#[cube]` macro can handle the largest MGGA kernels and whether f64 precision is maintained through the JIT compilation pipeline. Phase 2's canary kernel resolves this uncertainty early.

### Gaps to Address

- **CubeCL 0.9.0 f64 precision through JIT pipeline:** Must be validated empirically in Phase 2. No documentation confirms bit-exact f64 behavior through CubeCL's IR compilation.
- **CubeCL HIP backend maturity:** LOW confidence. Needs runtime testing on AMD hardware before committing to HIP support.
- **Automated translator feasibility:** The maple2c corpus has undocumented pattern variations. A full pattern catalog must be built before designing the translator in Phase 4.
- **Large kernel compilation limits:** The 5 largest MGGA kernels (50K-100K lines) may exceed GPU compiler limits. Must be tested early in Phase 4, not deferred to Phase 6.
- **Thread safety of CubeCL ComputeClient:** Must verify Send + Sync bounds. If not thread-safe, API design in Phase 5 must account for this.

## Sources

### Primary (HIGH confidence)
- Vendored libxc 7.0.0 source (libxc-master/) -- direct analysis of xc.h, gga.c, work_gga_inc.c, maple2c kernels, util.h
- Project design document (docs/design/libxc_rs_detailed_design.md) -- implementation-ready architecture
- CubeCL vendored documentation (docs/manual/Cubecl/) -- confirms f64 usage in #[cube] kernels
- Rust RFC 3514 Float Semantics -- IEEE 754 compliance guarantees

### Secondary (MEDIUM confidence)
- CubeCL GitHub/crates.io (0.9.0) -- API patterns and feature set
- WebGPU/wgpu f64 limitation discussions -- confirms WGPU f64 is not viable
- Rust community discussions on C-to-Rust floating-point divergence

### Tertiary (LOW confidence)
- CubeCL HIP backend status -- needs runtime validation
- CubeCL large kernel compilation behavior -- no documented limits, must test empirically
- Criterion 0.5 vs 0.8 stability -- pinned to 0.5.1 as conservative choice

---
*Research completed: 2026-04-09*
*Ready for roadmap: yes*
