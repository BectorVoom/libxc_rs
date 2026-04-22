# Requirements: libxc_rs

<<<<<<< HEAD
**Defined:** 2026-04-09
**Core Value:** Numerically accurate (energy relative error <= 10^-12 vs libxc oracle) evaluation of all 649 XC functionals from a single pure-Rust codebase that runs on both CPU and GPU without code duplication.

## v1 Requirements

Requirements for initial release. Each maps to roadmap phases.

### Domain Model

- [ ] **DOM-01**: All domain enums (Family, Kind, Spin, DerivativeOrder, HybridType, Dimensionality) defined with correct repr and derives
- [ ] **DOM-02**: FunctionalId newtype with validation via from_raw() and from_name()
- [ ] **DOM-03**: FunctionalFlags bitflags matching all libxc capability flags
- [ ] **DOM-04**: Dimensions struct computing correct array sizes for all family/spin combinations (up to 477 components for 4th-order polarized MGGA)
- [ ] **DOM-05**: Thresholds struct with correct defaults (density: 1e-15, zeta: 1e-10, sigma: 1e-24, tau: 1e-20)

### Static Registry

- [ ] **REG-01**: All 649 functional IDs present in registry with complete FunctionalMeta
- [ ] **REG-02**: O(1) lookup by ID via sparse array indexed by raw ID
- [ ] **REG-03**: O(log n) lookup by name via sorted slice with binary search
- [ ] **REG-04**: All 52 removed IDs return RemovedFunctionalId error with correct replacement ID and name
- [ ] **REG-05**: Library version/reference functions return correct static strings

### Error Handling

- [ ] **ERR-01**: LibxcRsError enum covers all error variants (unknown ID, removed ID, unsupported order, buffer mismatch, family mismatch, spin mismatch, ext param errors, GPU errors)
- [ ] **ERR-02**: All public API methods return Result<T, LibxcRsError>
- [ ] **ERR-03**: Evaluation is infallible after input validation passes (no runtime errors from kernels)

### Mathematical Core

- [ ] **MATH-01**: safe_cbrt handles negative values correctly (cbrt(-8) == -2, not NaN)
- [ ] **MATH-02**: pow_1_3, pow_2_3, pow_4_3, pow_5_3 implemented as #[cube] functions
- [ ] **MATH-03**: piecewise3 and piecewise5 implemented as branch-free #[cube] select operations
- [ ] **MATH-04**: erf and erfc approximations accurate to f64 precision (polynomial approximation)
- [ ] **MATH-05**: All mathematical constants (M_CBRT3, KF_CONST, RS_CONST, etc.) defined as f64 const
- [ ] **MATH-06**: Spin polarization transforms (to_total_zeta, spin_scaling, clamp_zeta) implemented
- [ ] **MATH-07**: DFT quantities (reduced_gradient_s, wigner_seitz_rs, tf_kinetic, dimensionless_alpha) implemented
- [ ] **MATH-08**: Polynomial and rational function evaluation via Horner's method
- [ ] **MATH-09**: All math core functions tested independently against known values and libm references
- [ ] **MATH-10**: Cross-backend consistency: same math function produces identical results on CPU and GPU

### Input/Output

- [ ] **IO-01**: LdaInput, GgaInput, MggaInput structs with buffer size validation against Dimensions
- [ ] **IO-02**: LdaOutput, GgaOutput, MggaOutput with Option<&mut [f64]> for NULL-pointer semantics
- [ ] **IO-03**: OutputMask bitflags for selecting derivative levels to compute
- [ ] **IO-04**: SoA interleaved buffer layout matching libxc convention (rho_a_0, rho_b_0, rho_a_1, ...)
- [ ] **IO-05**: MggaOutput supports all 70 derivative fields (1 + 4 + 10 + 20 + 35)

### CubeCL Kernel Substrate

- [ ] **KERN-01**: Kernel launch wrappers handle backend selection, buffer creation, CubeCount/CubeDim calculation
- [ ] **KERN-02**: LDA_X canary kernel passes oracle comparison at 10^-12 relative error (both spin modes)
- [x] **KERN-03**: All LDA kernel files translated from maple2c to #[cube] functions (~43 functionals)
- [ ] **KERN-04**: All GGA kernel files translated from maple2c to #[cube] functions (~130 functionals)
- [x] **KERN-05**: All MGGA kernel files translated from maple2c to #[cube] functions (~80 functionals)
- [x] **KERN-06**: Kernel translations preserve floating-point operation order from maple2c temporaries
- [x] **KERN-07**: Density thresholding: grid points below threshold skipped, spin densities clamped
- [x] **KERN-08**: Output accumulation via += for mixed functional support
- [x] **KERN-09**: Each functional/order/spin combination is a separate kernel function

### Evaluation Orchestration

- [ ] **EVAL-01**: Dispatch routes evaluation calls to correct kernel based on family, order, spin
- [ ] **EVAL-02**: Mixed functional accumulation: weighted sum of auxiliary functional results (matching mix_func.c)
- [ ] **EVAL-03**: EvaluationWorkspace pre-allocates scratch buffers for mixed functional evaluation
- [ ] **EVAL-04**: Non-mixed functionals require zero heap allocation in evaluation hot path
- [ ] **EVAL-05**: All hybrid/mixed functionals produce correct combined results

### Functional Instance

- [ ] **FUNC-01**: Functional::new(id, spin) constructs instance with correct metadata, dimensions, thresholds, ext_params
- [ ] **FUNC-02**: External parameter management: set/get by name, by index, bulk set/get
- [ ] **FUNC-03**: Threshold configuration: density, zeta, sigma, tau thresholds settable
- [ ] **FUNC-04**: Auxiliary functional initialization for hybrid/mixed functionals (recursive construction)
- [ ] **FUNC-05**: FunctionalParams trait for per-functional computed parameters derived from ext_params
- [ ] **FUNC-06**: Drop implementation cleans up resources

### High-Level API

- [ ] **API-01**: FunctionalBuilder with chained configuration (spin, thresholds, ext_params)
- [ ] **API-02**: BatchEvaluator with reusable workspace for repeated evaluations
- [ ] **API-03**: Ergonomic evaluate() method that dispatches by family automatically

### C Compatibility Layer

- [ ] **COMPAT-01**: All 85 public C API functions implemented as extern "C" functions
- [ ] **COMPAT-02**: C-compatible struct layouts for FFI consumers
- [ ] **COMPAT-03**: Unsafe code confined to compat/ module

### Hybrid Properties

- [ ] **HYB-01**: HybridType classification (Semilocal, Hybrid, Cam, CamYukawa, etc.)
- [ ] **HYB-02**: CAM coefficient extraction (omega, alpha, beta)
- [ ] **HYB-03**: NLC coefficient extraction (b, C)
- [ ] **HYB-04**: Auxiliary functional iteration (IDs and weights)

### GPU Support

- [ ] **GPU-01**: CubeCL CPU backend always available (cubecl-cpu)
- [ ] **GPU-02**: CUDA backend feature-gated (cubecl-cuda)
- [ ] **GPU-03**: HIP backend feature-gated (cubecl-hip)
- [ ] **GPU-04**: WGPU backend feature-gated with runtime f64 capability check
- [ ] **GPU-05**: GPU-resident buffer management (GpuBuffer<R>) minimizing host-device transfers
- [ ] **GPU-06**: f64-only precision policy: typed error if device lacks f64 support, no silent f32 fallback
- [ ] **GPU-07**: Backend selection from environment variable LIBXC_RS_BACKEND

### Oracle Verification

- [ ] **VERIFY-01**: Verification harness in verify/ crate using bindgen against system libxc 7.0.0
- [x] **VERIFY-02**: All 649 functionals verified against libxc oracle across applicable derivative orders and spin modes
- [x] **VERIFY-03**: Energy (exc): relative error <= 10^-12
- [x] **VERIFY-04**: VXC: relative error <= 10^-10
- [x] **VERIFY-05**: FXC: relative error <= 10^-8
- [x] **VERIFY-06**: KXC: relative error <= 10^-6
- [x] **VERIFY-07**: LXC: relative error <= 10^-4
- [ ] **VERIFY-08**: GPU results match CPU results to within 10^-14

### Performance

- [ ] **PERF-01**: CPU batch (1000 points): within 1.5x of libxc C
- [ ] **PERF-02**: GPU batch (100k points): > 5x CPU batch throughput
- [ ] **PERF-03**: Functional init (cold start): < 100 ms
- [ ] **PERF-04**: Benchmark suite with criterion for regression detection
- [ ] **PERF-05**: Zero heap allocation in non-mixed evaluation hot path

### Build Quality

- [ ] **BUILD-01**: cargo build succeeds with no warnings
- [ ] **BUILD-02**: cargo test passes all tests
- [ ] **BUILD-03**: cargo clippy has no warnings
- [ ] **BUILD-04**: No unsafe code outside compat/, kernel/launch.rs, and GPU buffer management
- [ ] **BUILD-05**: No runtime C/Fortran FFI dependency in the production library

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### Extended GPU

- **GPU-EXT-01**: Multi-GPU evaluation with work partitioning
- **GPU-EXT-02**: Async kernel launch with future-based result retrieval
- **GPU-EXT-03**: GPU memory pool with cross-functional buffer reuse

### Extended Features

- **FEAT-01**: VV10 non-local correlation kernel implementation
- **FEAT-02**: 1D/2D dimensionality evaluation modes
- **FEAT-03**: Automatic maple2c-to-Rust translator tool
- **FEAT-04**: Python bindings via PyO3
- **FEAT-05**: Streaming evaluation API for very large grids

### Documentation

- **DOC-01**: User guide with migration instructions from libxc C
- **DOC-02**: API documentation (rustdoc) for all public types and methods
- **DOC-03**: Performance tuning guide for GPU backends
=======
**Defined:** 2026-03-22
**Core Value:** Deliver full libxc public capability coverage through a safer Rust API without splitting CPU and GPU semantics into separate evaluator implementations.

## v1 Requirements

### Catalog and Metadata

- [ ] **CATL-01**: Caller can resolve every current public libxc functional by ID and get its family classification.
- [ ] **CATL-02**: Caller can resolve functionals by canonical name and supported legacy aliases.
- [ ] **CATL-03**: Caller can query functional metadata including family, kind, flags, references, derivative support, and external-parameter specifications.
- [ ] **CATL-04**: Generated artifacts preserve complete inventory coverage for the targeted libxc surface, including 85 public functions, 649 current IDs, and explicit legacy or removed identifier handling.

### Configuration and Validation

- [ ] **CONF-01**: Caller can construct a functional by ID or name with selected spin, thresholds, external parameters, precision policy, and runtime binding.
- [ ] **CONF-02**: Caller receives typed errors for invalid thresholds, unknown or removed identifiers, and invalid external-parameter names, indexes, or counts.
- [ ] **CONF-03**: Caller can submit LDA, GGA, and MGGA inputs through family-typed input bundles that validate shape and layout before launch.
- [ ] **CONF-04**: MGGA evaluation rejects missing `tau` or `lapl` channels before launch when metadata marks them as required.
- [ ] **CONF-05**: Caller can request only the needed derivative outputs and receive typed output bundles for derivative orders 0 through 4.

### Execution and Runtime

- [ ] **EXEC-01**: Host evaluation for LDA, GGA, and MGGA derivative orders 0 through 4 runs through CubeCL CPU kernels without a handwritten CPU evaluator.
- [ ] **EXEC-02**: The same kernel logic can execute on at least one GPU backend through CubeCL and returns typed backend-unavailable or capability-mismatch errors when unsupported.
- [ ] **EXEC-03**: Dispatch specializes by family, derivative order, spin mode, required MGGA channels, and precision policy while masking unused outputs.
- [ ] **EXEC-04**: Auxiliary, hybrid, and nonlocal-correlation accumulation flows run through the same device-side execution path instead of host-side formula fallbacks.
- [ ] **EXEC-05**: Resident execution keeps functionals, inputs, outputs, and scratch buffers resident across repeated launches and uploads only dirty input regions.

### API and Compatibility

- [ ] **API-01**: Safe Rust APIs cover lifecycle, metadata access, configuration, host evaluation, batch evaluation, and resident evaluation.
- [ ] **API-02**: Safe or compatibility APIs together reach all 85 public libxc functions from the targeted inventory.
- [ ] **API-03**: Compatibility shims preserve legacy aggregate evaluation entry points and packed libxc-compatible layout behavior for migration-oriented callers.
- [ ] **API-04**: Public library errors are exposed through `thiserror` v2 while verification, benchmarking, and CLI tooling can use `anyhow`.

### Verification and Performance

- [ ] **VERI-01**: Verification tooling compares Rust results against libxc across family, derivative order, spin mode, and supported runtime combinations.
- [ ] **VERI-02**: Verification reports include per-functional abs/rel/ULP metrics, CPU-vs-GPU parity summaries, and removed-identifier handling summaries.
- [ ] **PERF-01**: Benchmarks measure lookup, initialization, CPU batch, GPU batch, resident reuse, transfer volume, and cold-vs-warm execution behavior.
- [ ] **PERF-02**: Runtime caches and workspace reuse keep repeated evaluation paths free of avoidable hidden allocations and unnecessary transfers.

## v2 Requirements

### Runtime Expansion

- **RTEX-01**: Caller can target multiple GPU runtime backends with documented support coverage and parity status.
- **RTEX-02**: Caller can opt into additional autotuning and stream-placement policies beyond the initial stable runtime defaults.

### Precision and Forward Compatibility

- **PREC-01**: Caller can use optional `f32` or mixed-precision execution modes through a stable public API once parity thresholds are validated.
- **PREC-02**: Future concrete `LCA` or `OEP` functionals can be integrated without redesigning the public type model.
>>>>>>> origin/main

## Out of Scope

| Feature | Reason |
|---------|--------|
<<<<<<< HEAD
| Automatic differentiation (AD) for derivatives | XCFun showed AD is 1000x slower and suffers catastrophic error cancellation; maple2c hand-derived kernels are correct approach |
| f32 evaluation mode | Precision requirements (10^-12) mandate f64 throughout; mixed precision would be misleading |
| Runtime C header parsing | Pure Rust static data policy; no build-time dependency on libxc source |
| LCA/OEP functional families | libxc marks as deprecated/internal; negligible user demand |
| Real-time/interactive evaluation API | DFT is inherently batch; streaming deferred to v2 |
| WGPU as default backend | WebGPU spec lacks f64; must be opt-in with runtime capability check |

## Traceability

Which phases cover which requirements. Each requirement maps to exactly one phase.

| Requirement | Phase | Status |
|-------------|-------|--------|
| DOM-01 | Phase 1: Foundation and Registry | Pending |
| DOM-02 | Phase 1: Foundation and Registry | Pending |
| DOM-03 | Phase 1: Foundation and Registry | Pending |
| DOM-04 | Phase 1: Foundation and Registry | Pending |
| DOM-05 | Phase 1: Foundation and Registry | Pending |
| REG-01 | Phase 1: Foundation and Registry | Pending |
| REG-02 | Phase 1: Foundation and Registry | Pending |
| REG-03 | Phase 1: Foundation and Registry | Pending |
| REG-04 | Phase 1: Foundation and Registry | Pending |
| REG-05 | Phase 1: Foundation and Registry | Pending |
| ERR-01 | Phase 1: Foundation and Registry | Pending |
| ERR-02 | Phase 1: Foundation and Registry | Pending |
| ERR-03 | Phase 1: Foundation and Registry | Pending |
| VERIFY-01 | Phase 1: Foundation and Registry | Pending |
| BUILD-01 | Phase 1: Foundation and Registry | Pending |
| BUILD-02 | Phase 1: Foundation and Registry | Pending |
| BUILD-03 | Phase 1: Foundation and Registry | Pending |
| BUILD-04 | Phase 1: Foundation and Registry | Pending |
| BUILD-05 | Phase 1: Foundation and Registry | Pending |
| MATH-01 | Phase 2: Math Core and CubeCL Substrate | Pending |
| MATH-02 | Phase 2: Math Core and CubeCL Substrate | Pending |
| MATH-03 | Phase 2: Math Core and CubeCL Substrate | Pending |
| MATH-04 | Phase 2: Math Core and CubeCL Substrate | Pending |
| MATH-05 | Phase 2: Math Core and CubeCL Substrate | Pending |
| MATH-06 | Phase 2: Math Core and CubeCL Substrate | Pending |
| MATH-07 | Phase 2: Math Core and CubeCL Substrate | Pending |
| MATH-08 | Phase 2: Math Core and CubeCL Substrate | Pending |
| MATH-09 | Phase 2: Math Core and CubeCL Substrate | Pending |
| MATH-10 | Phase 2: Math Core and CubeCL Substrate | Pending |
| KERN-01 | Phase 2: Math Core and CubeCL Substrate | Pending |
| KERN-02 | Phase 2: Math Core and CubeCL Substrate | Pending |
| IO-01 | Phase 3: Input/Output and Evaluation Framework | Pending |
| IO-02 | Phase 3: Input/Output and Evaluation Framework | Pending |
| IO-03 | Phase 3: Input/Output and Evaluation Framework | Pending |
| IO-04 | Phase 3: Input/Output and Evaluation Framework | Pending |
| IO-05 | Phase 3: Input/Output and Evaluation Framework | Pending |
| EVAL-01 | Phase 3: Input/Output and Evaluation Framework | Pending |
| EVAL-02 | Phase 3: Input/Output and Evaluation Framework | Pending |
| EVAL-03 | Phase 3: Input/Output and Evaluation Framework | Pending |
| EVAL-04 | Phase 3: Input/Output and Evaluation Framework | Pending |
| EVAL-05 | Phase 3: Input/Output and Evaluation Framework | Pending |
| KERN-03 | Phase 4: Bulk Kernel Translation | Complete |
| KERN-04 | Phase 4: Bulk Kernel Translation | Pending |
| KERN-05 | Phase 4: Bulk Kernel Translation | Complete |
| KERN-06 | Phase 4: Bulk Kernel Translation | Complete |
| KERN-07 | Phase 4: Bulk Kernel Translation | Complete |
| KERN-08 | Phase 4: Bulk Kernel Translation | Complete |
| KERN-09 | Phase 4: Bulk Kernel Translation | Complete |
| VERIFY-02 | Phase 4: Bulk Kernel Translation | Complete |
| VERIFY-03 | Phase 4: Bulk Kernel Translation | Complete |
| VERIFY-04 | Phase 4: Bulk Kernel Translation | Complete |
| VERIFY-05 | Phase 4: Bulk Kernel Translation | Complete |
| VERIFY-06 | Phase 4: Bulk Kernel Translation | Complete |
| VERIFY-07 | Phase 4: Bulk Kernel Translation | Complete |
| FUNC-01 | Phase 5: Functional Lifecycle and Hybrid Properties | Pending |
| FUNC-02 | Phase 5: Functional Lifecycle and Hybrid Properties | Pending |
| FUNC-03 | Phase 5: Functional Lifecycle and Hybrid Properties | Pending |
| FUNC-04 | Phase 5: Functional Lifecycle and Hybrid Properties | Pending |
| FUNC-05 | Phase 5: Functional Lifecycle and Hybrid Properties | Pending |
| FUNC-06 | Phase 5: Functional Lifecycle and Hybrid Properties | Pending |
| HYB-01 | Phase 5: Functional Lifecycle and Hybrid Properties | Pending |
| HYB-02 | Phase 5: Functional Lifecycle and Hybrid Properties | Pending |
| HYB-03 | Phase 5: Functional Lifecycle and Hybrid Properties | Pending |
| HYB-04 | Phase 5: Functional Lifecycle and Hybrid Properties | Pending |
| API-01 | Phase 6: Public API and C Compatibility | Pending |
| API-02 | Phase 6: Public API and C Compatibility | Pending |
| API-03 | Phase 6: Public API and C Compatibility | Pending |
| COMPAT-01 | Phase 6: Public API and C Compatibility | Pending |
| COMPAT-02 | Phase 6: Public API and C Compatibility | Pending |
| COMPAT-03 | Phase 6: Public API and C Compatibility | Pending |
| GPU-01 | Phase 7: GPU Backends and Performance | Pending |
| GPU-02 | Phase 7: GPU Backends and Performance | Pending |
| GPU-03 | Phase 7: GPU Backends and Performance | Pending |
| GPU-04 | Phase 7: GPU Backends and Performance | Pending |
| GPU-05 | Phase 7: GPU Backends and Performance | Pending |
| GPU-06 | Phase 7: GPU Backends and Performance | Pending |
| GPU-07 | Phase 7: GPU Backends and Performance | Pending |
| VERIFY-08 | Phase 7: GPU Backends and Performance | Pending |
| PERF-01 | Phase 7: GPU Backends and Performance | Pending |
| PERF-02 | Phase 7: GPU Backends and Performance | Pending |
| PERF-03 | Phase 7: GPU Backends and Performance | Pending |
| PERF-04 | Phase 7: GPU Backends and Performance | Pending |
| PERF-05 | Phase 7: GPU Backends and Performance | Pending |

**Coverage:**
- v1 requirements: 83 total
- Mapped to phases: 83
- Unmapped: 0

---
*Requirements defined: 2026-04-09*
*Last updated: 2026-04-09 after roadmap creation*
=======
| Embedding libxc as the production evaluator | The runtime must remain Rust-native; libxc is reserved for oracle verification only |
| Separate handwritten CPU formula implementation | Conflicts with the CubeCL-only compute rule and increases semantic drift risk |
| C ABI structs as the primary user-facing API | The compatibility layer exists for reachability, not as the main ergonomic surface |
| Silent fallback from unsupported GPU runtime to unrelated execution path | Backend availability must fail explicitly so correctness and performance claims remain trustworthy |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| CATL-01 | Phase 1 | Pending |
| CATL-02 | Phase 1 | Pending |
| CATL-03 | Phase 1 | Pending |
| CATL-04 | Phase 1 | Pending |
| CONF-01 | Phase 2 | Pending |
| CONF-02 | Phase 2 | Pending |
| CONF-03 | Phase 2 | Pending |
| CONF-04 | Phase 2 | Pending |
| CONF-05 | Phase 2 | Pending |
| EXEC-01 | Phase 3 | Pending |
| EXEC-02 | Phase 3 | Pending |
| EXEC-03 | Phase 3 | Pending |
| EXEC-04 | Phase 3 | Pending |
| EXEC-05 | Phase 3 | Pending |
| API-01 | Phase 4 | Pending |
| API-02 | Phase 4 | Pending |
| API-03 | Phase 4 | Pending |
| API-04 | Phase 4 | Pending |
| VERI-01 | Phase 5 | Pending |
| VERI-02 | Phase 5 | Pending |
| PERF-01 | Phase 5 | Pending |
| PERF-02 | Phase 5 | Pending |

**Coverage:**
- v1 requirements: 22 total
- Mapped to phases: 22 ✓
- Unmapped: 0

---
*Requirements defined: 2026-03-22*
*Last updated: 2026-03-22 after roadmap creation*
>>>>>>> origin/main
