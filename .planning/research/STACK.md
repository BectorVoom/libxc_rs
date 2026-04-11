<<<<<<< HEAD
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
=======
# Stack Research

**Domain:** Rust numerical/scientific library (libxc_rs) with CubeCL CPU/GPU execution and a generated libxc API
**Researched:** 2026-03-22
**Confidence:** MEDIUM

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| CubeCL compute stack (cubecl + cubecl-core/cubecl-runtime + optional cubecl-cpu/cubecl-cuda/cubecl-hip/cubecl-wgpu + cubecl-ir) | 0.9.0 | Compile and auto-vectorize a single kernel set that runs on CPU and every GPU runtime with shared launch logic | CubeCL 0.9.0 documents show the CPU/GPU runtimes share the same compute substrate, and Burn 0.20’s release notes stress that CubeCL unifies CPU and GPU kernels so teams avoid duplicate formulas while keeping JIT/autotune optimizations per backend. citeturn0search0turn4search0 |
| bindgen | 0.72.0 | Auto-generate the Rust side of libxc’s hundreds of bezier-ed C APIs so code generation stays synchronized with upstream headers | The bindgen project and Effective Rust both advocate ‘let the compiler generate the bindings’ for large C APIs, and the packaged CLI is now at 0.72.0 in major distributions, which keeps regen tooling reproducible. citeturn7search2turn7search3turn4search5 |
| thiserror | 2.0 | Define typed, descriptive library errors that callers can convert into their own error graphs | Best-practice articles recommend `thiserror` to keep library boundaries strongly typed while leaving application layers free to wrap them with dynamic contexts. citeturn5search5turn9search7 |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| ndarray | 0.17.2 | Host-side multi-dimensional buffers, shared views, and chunkable slicing/zip helpers before handing data off to CubeCL | Latest releases emphasize lightweight views and chunking, which prevent extra allocations when shaping libxc’s rho/sigma/lapl/tau arrays. citeturn8search0turn8search1 |
| rayon | 1.11.0 | Parallelize layout validation, score computation, and verification batches before launching CubeCL kernels | Rayon’s work-stealing scheduler is the de facto way to express data-parallel preprocessing on Rust hosts, keeping CPU prep cheap. citeturn6search1 |
| anyhow | 1.0.102 | Propagate context-rich errors in verification tooling, benchmarks, and xtask automation without defining new enums | Guides on designing Rust error types encourage `anyhow` for applications so the verification harness can log details while keeping the library’s public error enum focused. citeturn5search5turn9search7 |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| cargo xtask pattern (referencing tracel-xtask 4.13.4 for reusable command helpers) | Orchestrate code generation, CubeCL kernel builds, verification runs, and benchmarks from workspace scripts | Matklad’s cargo-xtask README describes this pattern as cross-platform automation, and the recent tracel-xtask 4.13.4 release embodies a modern implementation that layers CLI helpers on top of `anyhow`, `clap`, and tracing-friendly crates. citeturn2search0turn10search0 |
| Criterion benchmarks | Stabilize throughput, latency, and transfer-cost measurements for CPU vs. GPU vs. resident flows | Criterion 0.8.2 is the current release for reproducible benchmarking, and performance guides push for consistent tooling because ad-hoc measurement will hide regressions. citeturn11search3turn6search0 |
| bindgen-cli | Regenerate bindings as xtask steps so the libxc inventory remains the source-of-truth after header changes | The CLI is the practical entry point for the bindgen pattern and is at version 0.72.0 in current packaging, matching the library release. citeturn7search2turn4search5 |
>>>>>>> origin/main

## Installation

```bash
<<<<<<< HEAD
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
=======
# Core tooling
cargo install bindgen-cli --version 0.72.0
cargo add cubecl@0.9.0 cubecl-core@0.9.0 cubecl-cpu@0.9.0 cubecl-cuda@0.9.0 cubecl-hip@0.9.0 cubecl-wgpu@0.9.0 cubecl-ir@0.9.0 cubecl-runtime@0.9.0

# Supporting libraries
cargo add ndarray@0.17.2 rayon@1.11.0 thiserror@2.0.18 anyhow@1.0.102

# Development/verification helpers
cargo add criterion@0.8.2 tracel-xtask@4.13.4
```

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| CubeCL unified kernels | Maintain separate WGPU/HIP/CUDA handwritten kernels | Only worth the added maintenance cost if you need very vendor-specific features that CubeCL cannot express (very rare now that Burn 0.20 uses CubeCL for both CPU and GPU). citeturn4search0turn4search2 |
| bindgen for ingoing libxc headers | cbindgen’s reverse direction | cbindgen shines when exposing a Rust API to C; for consuming a large C API like libxc, bindgen keeps the code in sync automatically. citeturn7search0turn6search1 |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Hand-rolled per-backend CUDA/HIP/Metal kernels | Duplicates logic and risks CPU/GPU drift while CubeCL’s 0.9.0 release plus the Burn 0.20/Phoronix coverage show teams consolidating kernels into one codebase | CubeCL gives a shared kernel infrared plus autotuning so you only write formulas once. citeturn0search0turn4search0 |
| cbindgen for libxc’s API | cbindgen is for generating C headers from Rust; using it to consume a C library would force you to reverse-engineer bindings manually | Use bindgen (and the Rust Patterns bindgen pattern) to import libxc’s inventory safely. citeturn6search1turn7search0 |

## Stack Patterns by Variant

**If you need to regenerate libxc bindings or run the verification harness against the upstream oracle:**
- Use the cargo-xtask pattern described by Matklad and the tracel-xtask helpers so you can run bindgen, compile CubeCL kernels, and run oracle comparisons from one script. citeturn2search0turn10search0
- Because the Rust Patterns FFI chapter explicitly calls out bindgen as the automation you want for large C APIs, pairing it with xtasks keeps the generated inventory always synced with the headers you validate against. citeturn7search0

**If you are filling resident inputs/outputs prior to CubeCL launches or prepping host-side verification data:**
- Use ndarray’s views (0.17.2) together with Rayon’s work-stealing pool for parallel validation, copying, or reading libxc/Oracle outputs before handing them to CubeCL. citeturn8search1turn6search1
- Because ndarray already provides zero-copy chunking and Rayon is the canonical host-level data-parallel pool, you avoid duplicating buffer logic and keep CPU prep consistent. citeturn8search0

## Version Compatibility

| Package | Compatible With | Notes |
|---------|-----------------|-------|
| cubecl 0.9.0 | cubecl-core, cubecl-runtime, cubecl-ir, cubecl-cpu/cuda/hip/wgpu all at 0.9.0 | Docs stress that the optional runtime crates are meant to be version-locked together so proc macros and runtimes generate consistent kernels. citeturn0search0 |
| bindgen 0.72.0 | bindgen-cli 0.72.0 | The SUSE packaging (0.72.0) mirrors the code generator release, which keeps build scripts deterministic. citeturn4search5 |
| tracel-xtask 4.13.4 | tracel-xtask-macros 4.13.4 / serde / clap etc. | The 4.13.4 release shines because it already depends on the same `anyhow`, `clap`, and logging crates we rely on, avoiding version conflicts. citeturn10search0 |

## Sources
- `cubecl` 0.9.0 docs — multi-platform runtime + optional CPU/GPU feature list. citeturn0search0
- Burn 0.20 release blog — CubeCL unifies CPU + GPU kernels for high performance. citeturn4search0
- Phoronix / Burn 0.20 coverage — CubeCL-based CubeK shows the practical benefit of a single kernel stack. citeturn4search1
- Burn end-of-year review — CubeCL CPU/Metal/MLIR expansion validates the cross-platform claim. citeturn4search2
- Matklad’s cargo-xtask README — layout for xtask automation. citeturn2search0
- tracel-xtask 4.13.4 docs — up-to-date reusable xtask helpers. citeturn10search0
- bindgen GitHub + documentation — auto-generate large C APIs. citeturn7search2
- Rust Patterns FFI chapter — bindgen pattern for massive C interfaces. citeturn7search0
- Effective Rust Item 35 — prefer bindgen over manual declarations. citeturn7search3
- SUSE bindgen 0.72.0 packaging note. citeturn4search5
- oneuptime/how-to guide — thiserror for libraries, anyhow for applications tooling. citeturn9search7
- Effective Rust Item 4 — typed enums for libraries; `thiserror` + `anyhow` distinctions. citeturn5search5
- ndarray 0.17.2 release notes — documented arrays for chunking + slicing. citeturn8search1
- Rayon data-parallel article — work-stealing best practice. citeturn6search1
- Criterion 0.8.2 docs — current benchmarking tool. citeturn11search3
- Rust performance guide on benchmarking — emphasize consistent tooling. citeturn6search0
- cbindgen documentation — alternative direction to avoid. citeturn6search1

---
*Stack research for: Rust numerical/scientific domain with CubeCL compute and generated libxc API.* 
>>>>>>> origin/main
