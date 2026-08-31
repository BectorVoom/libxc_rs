# libxc_rs

A Rust implementation of the libxc density functional theory (DFT) exchange-correlation library.

**Key Property:** Bit-exact against C libxc 7.0.0 (worst relative deviation ≤ 2.14e-16, under 1 ulp across LDA, GGA, MGGA, and hybrid functionals, with exact analytic and bit-identical agreement across multiple benchmark quantities).

- Safe, typed Rust API (`Functional`, `FunctionalBuilder`, `EvaluationWorkspace`)
- Parallel evaluation via Rayon (`libxc-reval`)
- Optional C-ABI compatibility layer (`libxc-compat`)
- Oracle-verified parity against C libxc 7.0.0

## 20. Source Tree

```
libxc_rs/
├── Cargo.toml                          # Workspace root
├── CLAUDE.md                           # AI assistant instructions
├── README.md                           # Project documentation
│
├── src/                                # Main library crate
│   ├── lib.rs                          # Public re-exports
│   │
│   ├── model/                          # Domain types
│   │   ├── mod.rs
│   │   ├── family.rs                   # Family enum
│   │   ├── kind.rs                     # Kind enum
│   │   ├── spin.rs                     # Spin enum
│   │   ├── derivative.rs              # DerivativeOrder enum
│   │   ├── id.rs                       # FunctionalId newtype
│   │   ├── flags.rs                    # FunctionalFlags bitflags
│   │   ├── dims.rs                     # Dimensions struct
│   │   ├── precision.rs               # Precision constants
│   │   └── thresholds.rs              # Thresholds struct
│   │
│   ├── meta/                           # Static metadata
│   │   ├── mod.rs
│   │   ├── functional_meta.rs         # FunctionalMeta struct
│   │   └── library.rs                 # Library version/reference
│   │
│   ├── registry/                       # Lookup tables
│   │   ├── mod.rs
│   │   └── tables.rs                  # Static ID→Meta, Name→ID tables
│   │
│   ├── error/                          # Error types
│   │   ├── mod.rs
│   │   ├── public.rs                  # LibxcRsError (thiserror v2)
│   │   ├── internal.rs                # Internal error helpers
│   │   └── ffi.rs                     # C-compatible error codes
│   │
│   ├── math/                           # Mathematical core
│   │   ├── mod.rs
│   │   ├── power.rs                   # pow_1_3, pow_2_3, safe_cbrt, etc.
│   │   ├── threshold.rs              # piecewise3, piecewise5, clamp, safe_div
│   │   ├── constants.rs              # Mathematical constants (M_CBRT3, etc.)
│   │   ├── spin_transform.rs         # to_total_zeta, spin_scaling, clamp_zeta
│   │   ├── special.rs                # erf_approx, erfc_approx
│   │   ├── polynomial.rs             # poly_eval, rational_eval (Horner)
│   │   └── dft_quantities.rs         # reduced_gradient_s, wigner_seitz_rs, etc.
│   │
│   ├── input/                          # Input bundles
│   │   ├── mod.rs
│   │   ├── lda.rs                     # LdaInput
│   │   ├── gga.rs                     # GgaInput
│   │   └── mgga.rs                    # MggaInput
│   │
│   ├── output/                         # Output bundles
│   │   ├── mod.rs
│   │   ├── mask.rs                    # OutputMask bitflags
│   │   ├── lda.rs                     # LdaOutput
│   │   ├── gga.rs                     # GgaOutput
│   │   └── mgga.rs                    # MggaOutput
│   │
│   ├── kernel/                         # CubeCL kernel implementations
│   │   ├── mod.rs
│   │   ├── launch.rs                  # Kernel launch wrappers
│   │   ├── shared/                    # Kernel-level shared code
│   │   │   ├── mod.rs
│   │   │   ├── spin.rs               # Spin handling in kernels
│   │   │   ├── thresholds.rs         # Density screening
│   │   │   └── output_mask.rs        # Conditional output writing
│   │   ├── lda/                       # LDA kernels (one file per functional)
│   │   │   ├── mod.rs
│   │   │   ├── lda_x.rs              # Slater exchange
│   │   │   ├── lda_c_vwn.rs          # VWN correlation
│   │   │   ├── lda_c_pw.rs           # PW correlation
│   │   │   └── ... (all LDA functionals)
│   │   ├── gga/                       # GGA kernels
│   │   │   ├── mod.rs
│   │   │   ├── gga_x_pbe.rs          # PBE exchange
│   │   │   ├── gga_c_lyp.rs          # LYP correlation
│   │   │   └── ... (all GGA functionals)
│   │   └── mgga/                      # MGGA kernels
│   │       ├── mod.rs
│   │       ├── mgga_x_scan.rs        # SCAN exchange
│   │       ├── mgga_c_tpss.rs        # TPSS correlation
│   │       └── ... (all MGGA functionals)
│   │
│   ├── eval/                           # Evaluation orchestration
│   │   ├── mod.rs
│   │   ├── dispatch.rs               # Family/order/spin dispatch
│   │   ├── mix.rs                    # Mixed functional accumulation
│   │   └── workspace.rs             # EvaluationWorkspace
│   │
│   ├── func/                           # Functional instance
│   │   ├── mod.rs
│   │   ├── lifecycle.rs              # new(), Drop
│   │   ├── config.rs                 # Threshold/ext_param setters
│   │   └── params.rs                 # FunctionalParams trait + impls
│   │
│   ├── hybrid/                         # Hybrid properties
│   │   ├── mod.rs
│   │   ├── cam.rs                    # CAM coefficients
│   │   ├── nlc.rs                    # Non-local correlation
│   │   └── auxiliary.rs              # Auxiliary functional access
│   │
│   ├── api/                            # High-level ergonomic API
│   │   ├── mod.rs
│   │   ├── builder.rs               # FunctionalBuilder
│   │   └── batch.rs                 # BatchEvaluator
│   │
│   ├── gpu/                            # GPU buffer management
│   │   ├── mod.rs
│   │   ├── buffer.rs                # GpuBuffer<R>
│   │   ├── pool.rs                  # Buffer pool / reuse
│   │   ├── backend.rs               # Backend selection + fallback
│   │   └── evaluator.rs            # GpuEvaluator
│   │
│   └── compat/                         # C compatibility layer
│       ├── mod.rs
│       └── ffi.rs                    # extern "C" functions
│
├── verify/                             # Oracle verification harness
│   ├── Cargo.toml                     # Dependencies: bindgen, anyhow, etc.
│   ├── build.rs                       # bindgen: libxc.h → FFI bindings
│   ├── src/
│   │   ├── main.rs                   # CLI entry point
│   │   ├── oracle.rs                 # libxc C FFI wrapper
│   │   ├── comparison.rs            # Result comparison logic
│   │   ├── test_data.rs             # BrOH/H/Li test system loading
│   │   └── report.rs                # JSON/HTML result output
│   └── tests/
│       └── oracle_comparison.rs      # Integration test: Rust vs C
│
├── benches/                            # Performance benchmarks
│   ├── lda_batch.rs                   # LDA throughput
│   ├── gga_batch.rs                   # GGA throughput
│   ├── mgga_batch.rs                  # MGGA throughput
│   ├── gpu_batch.rs                   # GPU batch throughput
│   ├── transfer_overhead.rs           # Host-device transfer cost
│   └── cold_start.rs                  # Functional::new() latency
│
├── xtask/                              # Build and development tasks
│   ├── Cargo.toml
│   └── src/
│       └── main.rs                   # cargo xtask commands
│
├── tests/                              # Integration tests
│   ├── api_coverage.rs               # All 85 public C functions are reachable
│   ├── registry_completeness.rs      # All 649 IDs resolve to metadata
│   ├── dimension_correctness.rs      # Dimension calculations match libxc
│   └── error_handling.rs            # Error variant coverage
│
├── docs/
│   ├── design/
│   │   └── libxc_rs_detailed_design.md  # THIS DOCUMENT
│   └── manual/
│       └── Cubecl/                   # CubeCL documentation
│
└── libxc-master/                       # Vendored libxc 7.0.0 source (reference only)
```