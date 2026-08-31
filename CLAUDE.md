## Project

**libxc_rs**

libxc_rs is a from-scratch Rust reimplementation of the libxc 7.0.0 exchange-correlation (XC) functional library used in density functional theory (DFT) calculations. It covers all 649 functionals across LDA, GGA, MGGA, and hybrid families, with derivatives through 4th order, polarized and unpolarized spin modes. Kernels are plain Rust over `&[f64]` slices, parallelised with rayon. The library targets computational chemists and DFT code developers who need a safe, performant XC functional library without C/Fortran FFI dependencies.

**CPU only.** 
Kernels are now generated straight from libxc's maple2c C by `tools/translate_rayon/from_maple.py`.
**Core Value:** Numerically accurate (energy relative error <= 10^-12 vs libxc oracle) evaluation of all 649 exchange-correlation functionals from a single pure-Rust codebase, with no C/Fortran in the production path.

### Constraints
- **Tech stack**: Pure Rust + rayon; no C/Fortran in production path. 
- **Precision**: f64 only; energy relative error <= 10^-12 vs libxc oracle
- **f32 support**: not a target. The kernels are f64-concrete by design — 2,892 emitted files over `&[f64]`, generated directly from libxc's maple2c C. Supporting f32 would mean re-architecting the translator to emit float-generic kernels, a full regen, and reconciling FP order. 
- **Dependencies**: rayon 1.11, wide 1.6 (explicit SIMD, pure Rust), rmath (vectorised libm, path dep at `~/Documents/workspace/rmath`), thiserror 2.0, bitflags 2.10, bytemuck 1.25 (production); bindgen, anyhow, criterion (verification/benchmark only)
- **Math policy**: rmath's **bit-exact path only**. Its top-level free functions are deliberately its `Fast` path, so the crate is taken as `rmath_upstream` and reached through `libxc_rkernel_math::rmath`, a shadow module pinning `<BitExact, FullRange>`. This is not stylistic: the tree ran 4-ulp `ln` against the 1e-12 contract until 2026-08-31 because both kernel forms called the same fast function and so agreed with each other. See `crates/kernels-rayon/math/src/rmath_bitexact.rs`.
- **Compatibility**: Must provide extern "C" layer for drop-in replacement in C/Fortran DFT codes
- **Operation order**: Maple2c formula translations must preserve floating-point operation order for bit-level equivalence


#### Current: plain Rust + rayon

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| rayon | 1.11 | Parallelism over grid points | Kernels are elementwise and embarrassingly parallel. Splitting is stride-aware (each output array has its own elements-per-point count) and done by recursive halving through `rayon::join`, so workers get disjoint `&mut` slices and the generated tree contains no `unsafe`. |
| (none) | - | Kernel authoring | Kernels are ordinary Rust functions over `&[f64]`. `ln`/`sqrt`/`exp`/`powf` resolve to the system libm — the same libm libxc calls — so this sits closer to the oracle than per-backend intrinsics did. |



### Core Production Dependencies
| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| bitflags | 2.10.0 | OutputMask, FunctionalFlags bitfield types | De facto standard for type-safe bitflags in Rust. Supports bytemuck derive via feature flag. Zero overhead. | HIGH |
| bytemuck | 1.25.0 | Byte/f64 slice casting | Used by the C-ABI shim and the verification harnesses. No longer needed for kernel dispatch — the rayon path passes `&[f64]` directly. | HIGH |
| thiserror | 2.0.18 | Typed error enums at library boundary | Standard for library error types. v2 supports `#[error(transparent)]`, automatic `provide()` for backtrace. 857M+ downloads. Use at public API boundary only. | HIGH |
| num-traits | 0.2.19 | `Float`, `FromPrimitive` traits for generic numeric code | Provides `Float::powi()`, `Float::cbrt()` etc. for CPU-side reference implementations and testing. 
### Verification / Test Dependencies (dev/build only)
| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| bindgen | 0.72.1 | Generate Rust FFI bindings from libxc C headers | Canonical tool for C FFI in Rust. Used in `verify/` crate to call libxc 7.0.0 as oracle. Already in project. | HIGH |
| cmake | 0.1.58 | Build vendored libxc C source in verify/ build script | Required to compile `libxc-master/` from source for oracle comparison. Already in project. | HIGH |
| anyhow | 1.0.100 | Ergonomic error handling in verify/bench/xtask | Standard for application-level errors. Use in tools only, never in library public API. | HIGH |
| approx | 0.5.1 | Float comparison assertions (`assert_relative_eq!`) | Provides `relative_eq!` and `ulps_eq!` macros for testing f64 values against oracle. Better than raw epsilon comparisons for scientific validation. | HIGH |
| criterion | 0.5.1 | Statistical benchmarking | De facto Rust benchmarking standard. Stable-compiler compatible. Produces statistical confidence intervals, regression detection, HTML reports. Use version 0.5.x (0.8.x is a recent rewrite -- verify stability before adopting). | MEDIUM |
| rayon | 1.11.0 | Parallel test execution in verify/ harness | Parallelize 10,312 regression tests across CPU cores. `par_iter()` over test cases with oracle comparison. Dev-dependency only. | HIGH |

## Installation
# Core production dependencies (already in Cargo.toml)
# dependency is rayon. 
| Setting | Value | Rationale |
|---------|-------|-----------|
| Edition | 2024 | Already set in Cargo.toml. Provides latest language features. |
| MSRV | 1.85.0+ | Edition 2024 requires Rust 1.85+. |
| Profile | release with `lto = "thin"` | Thin LTO balances compile time with cross-crate optimization for numerical code. |
## Key Technical Risks
| Risk | Severity | Mitigation |
|------|----------|------------|
| Splitting a merged kernel across functions loses its SIMD | HIGH | **The merged kernels do vectorise, and it is load-bearing.** Disassembly of four `lxc_pol` kernels (2026-08-17, release, `objdump`) puts 2-wide packed SSE at 10-68% of multiply slots: `gga_c_gapc` 68% mul / 66% add / 72% div (241 `divpd`), `mgga_x_br89` 24/24/33%, `gga_x_wpbeh` 22/23/10%, `mgga_c_r2scan` 11/11/16%. The share varies a lot per functional -- `gga_c_gapc` is the top of the range, not typical -- but it is never zero, and it is always SSE (`xmm` only, no AVX) from LLVM's *SLP* vectoriser, not loop vectorisation: the grid loop is not unrolled (one point per iteration). The value-merge is what earns it, by collecting many independent parts into one basic block. Splitting `gga_c_gapc`'s merged body across function boundaries dropped its packed arithmetic to 3-5% and cost 2.7x runtime (measured; runtime was only measured on that one functional, so expect the cost to scale with how much SIMD a given kernel had). Treat any change that puts a function boundary inside a merged output -- segmentation, cross-output sharing, `--cap` grouping -- as a runtime regression until disassembly says otherwise. The earlier note here ("loops do not vectorise, zero `divpd`, the 3-4x SIMD ceiling is unclaimed") described the pre-value-merge split-part tree and is no longer true. **Note the "always SSE, no AVX" observation was a build-flag artifact, corrected 2026-08-18:** the tree was compiling for baseline `x86-64` because a `[build] rustflags` entry is outranked by the `[target.'cfg(...)']` section in the developer's `~/.cargo/config.toml`. `.cargo/config.toml` now sets `-C target-cpu=native` in a `[target.'cfg(target_os = "linux")']` section, matching what libxc's own CMake (`ENABLE_XHOST`) does to the C oracle. See `docs/perf/vs-libxc.md`. With that flag the grid loop is loop-vectorised 8-wide (indexed `zmm` loads across grid points), not merely SLP-packed within one point; `docs/perf/kernel-codegen.md` has the disassembly and the two `objdump` mistakes that make this easy to misread. |
| `f64::mul_add` becomes a libm call without FMA | HIGH | `cbrt_f64` (and so every `pow_1_3`, several times per grid point) uses `mul_add`. At baseline `target-cpu` LLVM lowers that to `jmpq *fma@GOTPCREL`, a *software* fused-multiply-add, not one instruction. This was the single largest cost in the tree and is fixed by the `target-cpu=native` above; it is bit-exact, because hardware FMA and libm `fma()` are both correctly-rounded IEEE 754 `fusedMultiplyAdd`. Do not "fix" it by rewriting `mul_add` as `a*b+c` -- that changes the value. |
| 110 functionals unwired on ext_params | MEDIUM | Custom setters, C-expression defaults, or no libxc registration. Listed in `routing.rs::UNSUPPORTED` with reasons; they return `None` rather than run on guessed constants. |
| Criterion 0.5 vs 0.8 stability | LOW | Pin to 0.5.1 (proven stable). Evaluate 0.8.x in later phase. |
## Sources

Current:
- bindgen: https://crates.io/crates/bindgen (0.72.1)
- thiserror: https://crates.io/crates/thiserror (2.0.18)
- criterion: https://crates.io/crates/criterion
- rayon: https://docs.rs/crate/rayon/latest (1.11.0)
- approx: https://crates.io/crates/approx
- num-traits: https://crates.io/crates/num-traits (0.2.19)
- bitflags: https://crates.io/crates/bitflags (2.10.0)
- bytemuck: https://crates.io/crates/bytemuck

@AGENTS.md
