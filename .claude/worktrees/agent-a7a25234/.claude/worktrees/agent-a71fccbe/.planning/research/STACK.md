# Technology Stack

**Project:** libxc_rs -- Pure Rust DFT XC Functional Library with GPU Compute
**Researched:** 2026-04-09
**Overall confidence:** MEDIUM-HIGH (core stack verified; CubeCL f64/erf gaps flagged)

## Recommended Stack

### Compute Substrate (GPU + CPU)

| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| cubecl | 0.9.0 | Unified GPU/CPU kernel authoring | Only Rust crate providing single-source `#[cube]` kernels that compile to CUDA PTX, HIP, WGPU, and CPU SIMD from one codebase. Eliminates 649-functional duplication across backends. Active development by tracel-ai (Burn framework team). | HIGH |
| cubecl-cpu | 0.9.0 (via `cubecl` feature `cpu`) | CPU fallback and CI testing | Always-available backend; no GPU needed for correctness testing. Uses SIMD under the hood. | HIGH |
| cubecl-cuda | 0.9.0 (via `cubecl` feature `cuda`) | NVIDIA GPU execution | Feature-gated. CUDA backend compiles to PTX via cudarc. Full f64 support on compute-capable NVIDIA GPUs (sm_60+). | HIGH |
| cubecl-hip | 0.9.0 (via `cubecl` feature `hip`) | AMD GPU execution | Feature-gated. ROCm/HIP backend. Less mature than CUDA but functional. | LOW -- verify maturity |
| cubecl-wgpu | 0.9.0 (via `cubecl` feature `wgpu`) | Cross-platform GPU (Vulkan/Metal/DX12) | Feature-gated. **CRITICAL WARNING:** WebGPU spec does NOT include f64 in standard WGSL. wgpu exposes `SHADER_FLOAT64` as native-only feature. This backend may silently fail or require f64 emulation on many GPUs. Must return typed error if device lacks f64 support. | LOW -- f64 problematic |

**CubeCL f64 status:** The vendored docs confirm CubeCL works with `f64` types in `#[cube]` functions (see `cubecl_3d_dft.md` example using `f64` throughout). CPU backend handles f64 natively. CUDA backend supports f64 on capable hardware. WGPU f64 is the risk area.

**CubeCL erf/erfc gap:** CubeCL does NOT provide erf/erfc intrinsics. CUDA has native `__derf()` / `__derfc()` but CubeCL's JIT layer does not expose them. You MUST implement erf/erfc as pure `#[cube]` functions using rational approximation (Abramowitz & Stegun or minimax polynomial). This is feasible at f64 precision but adds implementation work.

**CubeCL cbrt gap:** No native `cbrt` in CubeCL. Implement as `copysign(pow(abs(x), 1.0/3.0), x)` to handle negative inputs correctly. The `pow_1_3` building block in the design doc addresses this.

### Core Production Dependencies

| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| bitflags | 2.10.0 | OutputMask, FunctionalFlags bitfield types | De facto standard for type-safe bitflags in Rust. Supports bytemuck derive via feature flag. Zero overhead. | HIGH |
| bytemuck | 1.25.0 | Safe casting between f64 slices and GPU byte buffers | Required for CubeCL `client.create(bytemuck::cast_slice(...))` pattern. derive feature enables `#[derive(Pod, Zeroable)]`. | HIGH |
| thiserror | 2.0.18 | Typed error enums at library boundary | Standard for library error types. v2 supports `#[error(transparent)]`, automatic `provide()` for backtrace. 857M+ downloads. Use at public API boundary only. | HIGH |
| num-traits | 0.2.19 | `Float`, `FromPrimitive` traits for generic numeric code | Provides `Float::powi()`, `Float::cbrt()` etc. for CPU-side reference implementations and testing. NOT used inside `#[cube]` kernels (CubeCL has its own type system). | MEDIUM |

### Verification / Test Dependencies (dev/build only)

| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| bindgen | 0.72.1 | Generate Rust FFI bindings from libxc C headers | Canonical tool for C FFI in Rust. Used in `verify/` crate to call libxc 7.0.0 as oracle. Already in project. | HIGH |
| cmake | 0.1.58 | Build vendored libxc C source in verify/ build script | Required to compile `libxc-master/` from source for oracle comparison. Already in project. | HIGH |
| anyhow | 1.0.100 | Ergonomic error handling in verify/bench/xtask | Standard for application-level errors. Use in tools only, never in library public API. | HIGH |
| approx | 0.5.1 | Float comparison assertions (`assert_relative_eq!`) | Provides `relative_eq!` and `ulps_eq!` macros for testing f64 values against oracle. Better than raw epsilon comparisons for scientific validation. | HIGH |
| criterion | 0.5.1 | Statistical benchmarking | De facto Rust benchmarking standard. Stable-compiler compatible. Produces statistical confidence intervals, regression detection, HTML reports. Use version 0.5.x (0.8.x is a recent rewrite -- verify stability before adopting). | MEDIUM |
| rayon | 1.11.0 | Parallel test execution in verify/ harness | Parallelize 10,312 regression tests across CPU cores. `par_iter()` over test cases with oracle comparison. Dev-dependency only. | HIGH |

### NOT Recommended

| Technology | Why Not | What Instead |
|------------|---------|-------------|
| ndarray | Adds unnecessary abstraction layer. libxc uses flat f64 slices, not N-d arrays. CubeCL has its own `Array<T>` type. Adding ndarray creates impedance mismatch with both CubeCL and libxc's C API. | Raw `&[f64]` / `&mut [f64]` slices matching libxc's buffer semantics |
| nalgebra | Linear algebra library for matrices/vectors. XC functionals operate on per-grid-point scalar values, not matrices. No matrix operations needed. | Direct scalar math in `#[cube]` kernels |
| rust-gpu (Embark) | Different approach: compiles full Rust to SPIR-V. Less mature for compute workloads. No CUDA backend. CubeCL is purpose-built for compute kernels. | CubeCL |
| opencl3 / ocl | OpenCL is legacy for new GPU compute. CubeCL covers Vulkan (via wgpu), CUDA, and HIP which supersede OpenCL. | CubeCL backends |
| snafu | More boilerplate than thiserror for equivalent functionality. thiserror is more widely adopted. | thiserror 2.0 |
| eyre | Application-focused error reporting (like anyhow with custom hooks). Overkill for library boundary. | thiserror at boundary, anyhow in tools |
| proptest / quickcheck | Property testing is useful generally but XC functional correctness is verified against deterministic oracle data, not random properties. The 10,312 regression tests with known inputs/outputs are more valuable. | Oracle-based regression tests with approx |
| divan | Newer benchmarking crate. Less ecosystem adoption than criterion. Criterion's statistical rigor is better for performance regression detection. | criterion |
| float-cmp | Similar to approx but less widely adopted. approx is the ecosystem standard. | approx |

## Installation

```bash
# Core production dependencies (already in Cargo.toml)
cargo add cubecl@0.9.0 --features cpu,wgpu
cargo add bitflags@2.10.0
cargo add bytemuck@1.25.0 --features derive
cargo add thiserror@2.0.18

# Optional GPU backends (feature-gated)
# In Cargo.toml:
# cubecl = { version = "0.9.0", features = ["cpu"] }
# [features]
# cuda = ["cubecl/cuda"]
# hip = ["cubecl/hip"]
# wgpu = ["cubecl/wgpu"]

# Verify crate (dev/build deps -- already configured)
# bindgen 0.72.1 (build-dep)
# cmake 0.1.58 (build-dep)
# anyhow 1.0.100

# Add missing dev dependencies
cargo add --dev approx@0.5.1
cargo add --dev criterion@0.5.1 --features html_reports
cargo add --dev rayon@1.11.0

# Optional: numeric traits for CPU reference
cargo add num-traits@0.2.19
```

## Feature Flag Architecture

```toml
[features]
default = ["cpu"]
cpu = ["cubecl/cpu"]         # Always available, CI default
cuda = ["cubecl/cuda"]       # NVIDIA GPUs (sm_60+ for f64)
hip = ["cubecl/hip"]         # AMD GPUs via ROCm
wgpu = ["cubecl/wgpu"]       # Vulkan/Metal/DX12 (f64 support varies)
c-api = []                   # Build extern "C" compatibility layer
```

**Rationale:** GPU backends are heavy compile-time dependencies. Default to CPU-only for fast CI. Users opt into GPU backends for their target hardware. The `c-api` feature gates the extern "C" layer so pure-Rust consumers don't pay for it.

## Rust Edition and Toolchain

| Setting | Value | Rationale |
|---------|-------|-----------|
| Edition | 2024 | Already set in Cargo.toml. Provides latest language features. |
| MSRV | 1.85.0+ | Edition 2024 requires Rust 1.85+. CubeCL 0.9.0 likely requires similar. |
| Profile | release with `lto = "thin"` | Thin LTO balances compile time with cross-crate optimization for numerical code. |

```toml
[profile.release]
lto = "thin"
codegen-units = 1
opt-level = 3

[profile.bench]
inherits = "release"
debug = true  # Enable profiling symbols
```

## Key Technical Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| CubeCL lacks erf/erfc intrinsics | HIGH | Implement as pure `#[cube]` rational approximation. Verify against libm reference to 10^-15 relative error. Test early in Phase 2. |
| WGPU backend lacks f64 on many GPUs | MEDIUM | Return `Error::F64NotSupported` at runtime device query. Document CUDA as primary GPU target. WGPU is best-effort. |
| CubeCL 0.9.0 kernel compilation limits | MEDIUM | Some MGGA 4th-order polarized kernels produce massive IR. May need kernel splitting. Benchmark compilation times early. |
| CubeCL `ComputeClient` thread safety | LOW | Verify `Send + Sync` bounds. If not thread-safe, wrap in `Arc<Mutex<>>` or use per-thread clients. |
| Criterion 0.5 vs 0.8 stability | LOW | Pin to 0.5.1 (proven stable). Evaluate 0.8.x in later phase. |

## Sources

- CubeCL GitHub: https://github.com/tracel-ai/cubecl (0.9.0 confirmed on crates.io)
- CubeCL crates.io: https://crates.io/crates/cubecl
- WebGPU f64 issue: https://github.com/gpuweb/gpuweb/issues/2805
- wgpu SHADER_FLOAT64 feature: https://docs.rs/wgpu-types/latest/wgpu_types/struct.Features.html
- bindgen: https://crates.io/crates/bindgen (0.72.1)
- thiserror: https://crates.io/crates/thiserror (2.0.18)
- criterion: https://crates.io/crates/criterion
- rayon: https://docs.rs/crate/rayon/latest (1.11.0)
- approx: https://crates.io/crates/approx
- num-traits: https://crates.io/crates/num-traits (0.2.19)
- bitflags: https://crates.io/crates/bitflags (2.10.0)
- bytemuck: https://crates.io/crates/bytemuck
- Vendored CubeCL docs: `docs/manual/Cubecl/cubecl_3d_dft.md` (confirms f64 usage in #[cube] kernels)
