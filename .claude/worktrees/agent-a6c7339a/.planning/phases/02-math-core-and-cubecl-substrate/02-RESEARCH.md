# Phase 2: Math Core and CubeCL Substrate - Research

**Researched:** 2026-04-09
**Domain:** CubeCL GPU compute substrate, numerical math functions, maple2c kernel translation
**Confidence:** HIGH

## Summary

Phase 2 establishes the CubeCL compute substrate and mathematical building blocks that all subsequent kernel phases depend on. The phase adds `cubecl` (CPU backend only) as a dependency, implements ~20 math core functions as `#[cube]` annotated functions, translates the LDA_X canary kernel (1,434 lines of C across 10 functions: 5 derivative orders x 2 spin modes), and builds the kernel launch infrastructure.

CubeCL 0.9.0 (latest stable, published January 2026) provides most math operations needed (`Sqrt`, `Abs`, `Powf`, `Exp`, `Log`, `Sin`, `Cos`, `Erf`) but critically lacks a `Cbrt` intrinsic. The `safe_cbrt` function must be hand-implemented using `sign * |x|^(1/3)` via `Abs` + `Powf`. CubeCL's built-in `Erf` trait exists for f64 but its precision is unverified -- per locked decision D-05/D-06, we implement our own Cephes-style piecewise rational approximation regardless. The `select()` function provides branchless conditional execution needed for `piecewise3`/`piecewise5` translation.

The LDA_X canary kernel is structurally simple (scalar arithmetic on density, no gradient or kinetic terms) but exercises the full translation pattern: maple2c temporaries (t2, t3, ...), `POW_1_3`, `my_piecewise3`, `M_CBRT3/M_CBRTPI/M_CBRT2` constants, `params->alpha` external parameter passthrough, and `+=` output accumulation across all 5 derivative orders.

**Primary recommendation:** Implement math core first with full test coverage, then build launch infrastructure, then translate LDA_X -- each step validates the one before it.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Add cubecl with only the `cpu` feature in Phase 2. GPU backends (cuda, hip, wgpu) are feature-gated and deferred to Phase 7.
- **D-02:** Trust the CubeCL CPU backend directly for testing -- no separate plain-Rust reference implementations. Tests run math functions through CubeCL CPU and compare against hardcoded known values and libm sweeps.
- **D-03:** Flat `src/math/` module with submodules: `powers.rs`, `piecewise.rs`, `constants.rs`, `spin.rs`, `erf.rs`, `dft_quantities.rs`, `polynomials.rs`. All functions are `#[cube]`-annotated.
- **D-04:** LDA_X canary kernel lives at `src/kernel/lda/lda_x.rs`. Creates the `kernel/` module hierarchy that Phase 4 populates.
- **D-05:** Use Cephes/libm-style piecewise rational approximation for erf and erfc.
- **D-06:** Target full f64 precision (~1e-15 relative error) for erf/erfc.
- **D-07:** Tests live inline as `#[cfg(test)]` at the bottom of each math submodule.
- **D-08:** Test against both hardcoded known values AND libm sweep tests. Add `libm` as a dev-dependency.
- **D-09:** Cross-backend consistency testing (MATH-10) deferred to Phase 7. Phase 2 verifies CubeCL CPU only.
- **D-10:** Translate all derivative orders through 4th (exc, vxc, fxc, kxc, lxc) from lda_x.c.
- **D-11:** Include both unpolarized and polarized spin modes.
- **D-12:** Manual hand-translation preserving exact variable names (t2, t3, ...) and floating-point operation order.
- **D-13:** Build full launch infrastructure in `kernel/launch.rs` -- backend selection, buffer management, CubeCount/CubeDim calculation, dispatch traits.

### Claude's Discretion
- Exact CubeCL `ComputeClient` initialization pattern and lifetime management
- CubeCount/CubeDim calculation strategy (elements per workgroup)
- Whether `poly_eval` and `rational_eval` use const generics or slices for coefficient arrays
- Internal organization of kernel/launch.rs (traits, structs, helper functions)
- libm dev-dependency version selection

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| MATH-01 | safe_cbrt handles negative values correctly (cbrt(-8) == -2, not NaN) | CubeCL lacks Cbrt intrinsic; implement via sign(x) * abs(x).powf(1.0/3.0). Verified CubeCL has Abs + Powf traits for f64. |
| MATH-02 | pow_1_3, pow_2_3, pow_4_3, pow_5_3 as #[cube] functions | Build on safe_cbrt. Match libxc util.h pattern: pow_2_3 = cbrt(x)*cbrt(x), pow_4_3 = x*cbrt(x), etc. |
| MATH-03 | piecewise3 and piecewise5 as branch-free #[cube] select operations | CubeCL provides `select(cond, then, or_else)` -- direct mapping from `my_piecewise3(c, x1, x2)`. |
| MATH-04 | erf and erfc accurate to f64 precision | Locked: Cephes/libm-style piecewise rational approx. CubeCL has Erf trait but precision unverified. Implement custom per D-05/D-06. |
| MATH-05 | All mathematical constants defined as f64 const | Extract from libxc util.h: M_CBRT2..M_CBRT9, M_CBRTPI, M_SQRTPI, X_FACTOR_C. Truncate long double literals to f64. |
| MATH-06 | Spin polarization transforms | Design doc Section 7.2.4: to_total_zeta, spin_scaling, clamp_zeta as #[cube] functions. |
| MATH-07 | DFT quantities (reduced_gradient_s, wigner_seitz_rs, tf_kinetic, dimensionless_alpha) | Design doc Section 7.2.7. Not needed by LDA_X canary but required by GGA/MGGA in Phase 4. |
| MATH-08 | Polynomial and rational function evaluation via Horner's method | Used by erf implementation internally and by GGA/MGGA functionals later. #[cube] compatible. |
| MATH-09 | All math core functions tested independently | Locked: inline #[cfg(test)], hardcoded known values + libm sweep comparison per D-07/D-08. |
| MATH-10 | Cross-backend consistency (CPU vs GPU identical) | Deferred to Phase 7 per D-09. Phase 2 verifies CubeCL CPU only. |
| KERN-01 | Kernel launch wrappers handle backend selection, buffer creation, CubeCount/CubeDim | CubeCL patterns verified from vendored docs: ComputeClient::load(), client.create(), ArrayArg::from_raw_parts(), launch_unchecked::<CpuRuntime>(). |
| KERN-02 | LDA_X canary kernel passes oracle comparison at 10^-12 relative error | lda_x.c analyzed: 1434 lines, 10 functions. Uses POW_1_3, my_piecewise3, M_CBRT3/M_CBRTPI/M_CBRT2 constants, params->alpha. Verify crate oracle infrastructure exists. |
</phase_requirements>

## Project Constraints (from CLAUDE.md)

- **Tech stack**: Pure Rust + CubeCL 0.9.0; no C/Fortran in production path
- **Precision**: f64 only; energy relative error <= 10^-12 vs libxc oracle
- **Edition**: 2024, `#![deny(warnings)]` enforced
- **Dependencies**: cubecl 0.9.0 (production); libm (dev-dependency for testing)
- **Operation order**: Maple2c formula translations must preserve floating-point operation order
- **No unsafe outside**: compat/, kernel/launch.rs, and GPU buffer management
- **Module-per-directory structure** established in Phase 1

## Standard Stack

### Core (New in Phase 2)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| cubecl | 0.9.0 | Unified GPU/CPU kernel authoring | Only Rust crate for single-source #[cube] kernels. Latest stable release (Jan 2026). [VERIFIED: docs.rs/crate/cubecl/latest] |
| cubecl-core | 0.9.0 | Core types: Array, CubeDim, CubeCount, #[cube] macro | Re-exported through cubecl. Provides Float, Abs, Sqrt, Powf, Erf, select(). [VERIFIED: docs.rs/cubecl-core/0.9.0/cubecl_core/frontend] |
| cubecl-cpu | 0.9.0 | CPU backend via MLIR/LLVM JIT | Always-available backend, no GPU needed. CpuDevice, CpuRuntime. [VERIFIED: vendored docs cubecl_3d_dft.md] |

### Dev Dependencies (New in Phase 2)
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| libm | 0.2.16 | Reference erf/erfc/cbrt for sweep tests | Dev-dependency only for math core validation. [VERIFIED: crates.io search] |

### Already Present (from Phase 1)
| Library | Version | Purpose |
|---------|---------|---------|
| bitflags | 2.10.0 | FunctionalFlags |
| bytemuck | 1.25.0 | Safe f64 <-> byte buffer casting for CubeCL client.create() |
| thiserror | 2.0.18 | Error types |
| anyhow | 1.0.100 | verify/ crate error handling |
| approx | 0.5.1 | Float comparison in verify/ tests |
| bindgen | 0.72.1 | verify/ build.rs FFI generation |
| cmake | 0.1.58 | verify/ build.rs libxc compilation |

**Cargo.toml changes needed:**
```toml
[dependencies]
cubecl = { version = "0.9.0", default-features = false }
cubecl-core = "0.9.0"
cubecl-cpu = "0.9.0"

[dev-dependencies]
libm = "0.2.16"

[features]
default = ["cpu"]
cpu = []  # cubecl-cpu is always a dependency in Phase 2; feature exists for future gating
cuda = []  # placeholder, Phase 7
hip = []   # placeholder, Phase 7
wgpu = []  # placeholder, Phase 7
```

Note: The exact Cargo.toml structure for cubecl features needs validation. The cubecl crate uses feature flags like `cubecl = { version = "0.9.0", features = ["cpu"] }` per CLAUDE.md. [ASSUMED]

## Architecture Patterns

### Recommended Project Structure (New Modules)
```
src/
+-- math/
|   +-- mod.rs           # pub mod declarations
|   +-- constants.rs     # f64 const values (M_CBRT3, etc.)
|   +-- powers.rs        # safe_cbrt, pow_1_3, pow_2_3, pow_4_3, pow_5_3
|   +-- piecewise.rs     # piecewise3, piecewise5 via select()
|   +-- erf.rs           # erf_approx, erfc_approx (Cephes-style)
|   +-- polynomials.rs   # poly_eval, rational_eval (Horner's method)
|   +-- spin.rs          # to_total_zeta, spin_scaling, clamp_zeta
|   +-- dft_quantities.rs # reduced_gradient_s, wigner_seitz_rs, tf_kinetic, dimensionless_alpha
+-- kernel/
|   +-- mod.rs           # pub mod declarations
|   +-- launch.rs        # Backend selection, buffer mgmt, CubeCount/CubeDim
|   +-- lda/
|       +-- mod.rs
|       +-- lda_x.rs     # Canary kernel: 10 functions (5 orders x 2 spins)
```

### Pattern 1: CubeCL Math Function
**What:** Every math core function is a `#[cube]` function that takes/returns f64 scalars.
**When to use:** All shared mathematical operations.
**Example:**
```rust
// Source: Design doc Section 7.2.1, CubeCL docs.rs frontend traits
use cubecl_core::prelude::*;

#[cube]
pub fn safe_cbrt(x: f64) -> f64 {
    // CubeCL has no Cbrt intrinsic. Implement as sign * |x|^(1/3).
    // powf(negative, 1/3) returns NaN, so we must use abs first.
    let abs_x = f64::abs(x);
    let result = f64::powf(abs_x, 1.0 / 3.0);
    // Preserve sign: if x < 0, negate result
    select(x < 0.0, -result, result)
}

#[cube]
pub fn pow_1_3(x: f64) -> f64 {
    // Matches libxc: POW_1_3(x) = cbrt(x)
    // For DFT, x (density) is always >= 0, but safe_cbrt handles negatives
    safe_cbrt(x)
}

#[cube]
pub fn pow_2_3(x: f64) -> f64 {
    let c = safe_cbrt(x);
    c * c
}

#[cube]
pub fn pow_4_3(x: f64) -> f64 {
    x * safe_cbrt(x)
}

#[cube]
pub fn pow_5_3(x: f64) -> f64 {
    let c = safe_cbrt(x);
    x * c * c
}
```

### Pattern 2: Piecewise via select()
**What:** Map maple2c's `my_piecewise3(c, x1, x2)` to CubeCL's `select(c, x1, x2)`.
**When to use:** Every maple2c kernel uses these for threshold guards.
**Example:**
```rust
// Source: CubeCL docs.rs select function, libxc util.h line 109
use cubecl_core::prelude::*;

#[cube]
pub fn piecewise3(cond: bool, val_true: f64, val_false: f64) -> f64 {
    select(cond, val_true, val_false)
}

#[cube]
pub fn piecewise5(c1: bool, v1: f64, c2: bool, v2: f64, v_else: f64) -> f64 {
    select(c1, v1, select(c2, v2, v_else))
}
```

### Pattern 3: CubeCL Kernel Launch (from vendored docs)
**What:** Buffer creation, kernel dispatch, result readback pattern.
**When to use:** All kernel evaluations.
**Example:**
```rust
// Source: docs/manual/Cubecl/cubecl_3d_dft.md lines 205-280
use cubecl_cpu::{CpuDevice, CpuRuntime};
use cubecl_core::prelude::*;
use cubecl_runtime::client::ComputeClient;

let device = CpuDevice::default();
let client: ComputeClient<_> = ComputeClient::load(&device);

// Upload input
let rho_handle = client.create(bytemuck::cast_slice(&rho_data));
// Allocate output
let zk_handle = client.empty(np * core::mem::size_of::<f64>());

let cube_dim = CubeDim::new(256, 1, 1);
let cube_count = CubeCount::Static((np as u32 + 255) / 256, 1, 1);

unsafe {
    lda_x_exc_unpol::launch_unchecked::<CpuRuntime>(
        &client,
        cube_count,
        cube_dim,
        ArrayArg::from_raw_parts::<f64>(&rho_handle, np, 1),
        ArrayArg::from_raw_parts::<f64>(&zk_handle, np, 1),
        alpha,           // scalar param
        dens_threshold,  // scalar param
        zeta_threshold,  // scalar param
    );
}

// Read back
let zk_bytes = client.read_one(zk_handle);
let zk = f64::from_bytes(&zk_bytes);
```

### Pattern 4: Maple2c Translation Pattern
**What:** Direct C-to-Rust translation preserving variable names and operation order.
**When to use:** All 270 kernel file translations (canary LDA_X first, bulk in Phase 4).
**Example:**
```rust
// Source: libxc-master/src/maple2c/lda_exc/lda_x.c lines 19-42
// C original:
//   t2 = rho[0] / 0.2e1 <= p->dens_threshold;
//   t3 = M_CBRT3;
//   t6 = t3 / t4;
//   t11 = POW_1_3(rho[0]);
//   t15 = my_piecewise3(t2, 0, -0.3e1 / 0.8e1 * t6 * t10 * t11);

#[cube(launch_unchecked)]
pub fn lda_x_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip >= zk.len() { return; }

    let t2 = rho[ip] / 0.2e1 <= dens_threshold;
    let t3 = M_CBRT3;                    // const from math::constants
    let t4 = M_CBRTPI;                   // const from math::constants
    let t6 = t3 / t4;
    let t8 = pow_1_3(zeta_threshold);    // math::powers::pow_1_3
    let t10 = piecewise3(1.0 <= zeta_threshold, t8 * zeta_threshold, 1.0);
    let t11 = pow_1_3(rho[ip]);
    let t15 = piecewise3(t2, 0.0, -0.3e1 / 0.8e1 * t6 * t10 * t11);
    let t16 = alpha * t15;
    let tzk0 = 0.2e1 * t16;

    zk[ip] += tzk0;
}
```

### Anti-Patterns to Avoid
- **Trait objects in math core:** Never use `dyn Float` or trait objects. All math functions are concrete `#[cube] fn(f64) -> f64`. CubeCL inlines them at kernel compilation.
- **Separate CPU reference implementation:** Per D-02, do NOT write parallel plain-Rust implementations. Test via CubeCL CPU backend directly.
- **Reordering operations in translation:** Maple2c temporaries encode specific floating-point evaluation order. Changing `a * b * c` to `a * (b * c)` changes results at the ULP level.
- **Using CubeCL's built-in Erf for precision-critical code:** Per D-05/D-06, implement custom Cephes-style erf. CubeCL's built-in Erf precision is unverified.
- **Forgetting `+= ` accumulation:** All output writes use `+=`, not `=`. This supports mixed functional accumulation (KERN-08, Phase 4).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Branchless select | Custom bit-manipulation select | `cubecl_core::frontend::select()` | CubeCL optimizes per-backend; handles type expansion correctly |
| Buffer byte casting | Manual pointer arithmetic | `bytemuck::cast_slice()` | Safe, zero-cost, already in deps |
| Float comparison in tests | `(a - b).abs() < epsilon` | `approx::assert_relative_eq!` | Handles relative vs absolute tolerance correctly |
| Workgroup size calculation | Manual `(n + size - 1) / size` | `cubecl::calculate_cube_count_elemwise` if available | Built-in utility handles edge cases [ASSUMED -- verify availability] |
| f64 byte conversion | `unsafe { std::slice::from_raw_parts() }` | `f64::from_bytes()` / `f64::as_bytes()` from cubecl | CubeCL provides these methods as shown in vendored docs |

**Key insight:** CubeCL handles the IR generation, backend compilation, and buffer management. The implementation work is (1) writing the `#[cube]` functions with correct math and (2) setting up the launch boilerplate correctly.

## Common Pitfalls

### Pitfall 1: cbrt(-x) Returns NaN via powf
**What goes wrong:** `(-8.0_f64).powf(1.0/3.0)` returns `NaN` in IEEE 754, not `-2.0`.
**Why it happens:** Fractional powers of negative numbers are undefined in real arithmetic for `powf`. C's `cbrt()` is a special function that handles sign.
**How to avoid:** `safe_cbrt` extracts sign, computes `|x|^(1/3)`, restores sign. This is the fundamental reason MATH-01 exists as a separate requirement.
**Warning signs:** Any test with negative density derivatives will produce NaN.

### Pitfall 2: Maple2c Literal Precision
**What goes wrong:** C literal `0.3e1` is `double 3.0`, but careless Rust translation might use `3_i32 as f64` or integer division.
**Why it happens:** Maple2c uses Maple's output format: `0.3e1` = 3.0, `0.8e1` = 8.0, `0.2e1` = 2.0. The division `-0.3e1 / 0.8e1` is `-3.0 / 8.0 = -0.375`.
**How to avoid:** Translate all maple2c numeric literals directly as f64 constants. `0.3e1` becomes `3.0_f64`. Never convert through integer arithmetic.
**Warning signs:** Relative error > 10^-12 that traces back to a constant evaluation.

### Pitfall 3: CubeCL select() Evaluates Both Branches
**What goes wrong:** Both branches execute even when condition is false. If a branch computes `1.0 / x` where `x == 0.0`, you get `Inf` even though that branch "shouldn't execute."
**Why it happens:** `select()` is branchless -- both sides are computed, then one is selected.
**How to avoid:** Ensure both branches of `piecewise3`/`piecewise5` produce finite values for any input. The maple2c code is already designed for this (threshold guards produce 0 in the "skip" branch).
**Warning signs:** NaN or Inf in output at grid points near density threshold.

### Pitfall 4: Output Accumulation Must Start From Zero
**What goes wrong:** Kernel writes `zk[ip] += result` but the output buffer was not zero-initialized.
**Why it happens:** `client.empty()` allocates uninitialized memory.
**How to avoid:** Zero-initialize output buffers before kernel launch. Either use `client.create(bytemuck::cast_slice(&vec![0.0f64; np]))` or zero explicitly.
**Warning signs:** Non-deterministic output values that differ between runs.

### Pitfall 5: Long Double Constants Truncated to f64
**What goes wrong:** Libxc util.h defines `M_CBRT3 = 1.442249570307408382321638310780109588392L` (long double, ~34 digits). Rust f64 holds ~15-17 significant digits.
**Why it happens:** f64 has 52-bit mantissa = ~15.9 decimal digits of precision.
**How to avoid:** Truncate constants to 17 significant digits (enough for exact f64 round-trip). Use `1.4422495703074084_f64` not the full long double literal. Validate each constant against libm reference.
**Warning signs:** Last-ULP differences that accumulate through derivative chains.

### Pitfall 6: CubeCL ABSOLUTE_POS Out-of-Bounds
**What goes wrong:** Kernel launches with `CubeCount` that rounds up to next workgroup boundary. Last workgroup has threads beyond array length.
**Why it happens:** Standard GPU pattern: `(np + 255) / 256` workgroups of 256 threads = up to 255 excess threads.
**How to avoid:** Every kernel MUST start with `if ip >= array.len() { return; }` or `terminate!()` guard.
**Warning signs:** Buffer overrun, undefined behavior, wrong results in last elements.

### Pitfall 7: Polarized LDA_X Has 2x Density Components
**What goes wrong:** Unpolarized uses `rho[ip]` as total density. Polarized uses `rho[2*ip]` and `rho[2*ip+1]` as spin-up and spin-down.
**Why it happens:** Libxc interleaves spin components: `[rho_a_0, rho_b_0, rho_a_1, rho_b_1, ...]`.
**How to avoid:** Polarized kernel must index `rho[ip * 2]` and `rho[ip * 2 + 1]`. Unpolarized uses `rho[ip]`. Buffer sizes differ: unpolarized `np` elements, polarized `2*np` elements.
**Warning signs:** Off-by-factor-of-2 errors, accessing wrong spin component.

## Code Examples

### CubeCL ComputeClient Initialization
```rust
// Source: docs/manual/Cubecl/cubecl_3d_dft.md line 207-208
use cubecl_cpu::{CpuDevice, CpuRuntime};
use cubecl_runtime::client::ComputeClient;

let device = CpuDevice::default();
let client: ComputeClient<_> = ComputeClient::load(&device);
```

### CubeCL Buffer Creation and Readback
```rust
// Source: docs/manual/Cubecl/cubecl_3d_dft.md lines 224-282
// Upload host data to device
let handle = client.create(f64::as_bytes(&host_data));

// Allocate empty buffer on device
let out_handle = client.empty(np * core::mem::size_of::<f64>());

// Read back after kernel execution
let bytes = client.read_one(out_handle);
let result = f64::from_bytes(&bytes);
```

### Erf Piecewise Rational Approximation Structure
```rust
// Source: libm erf.c (Cephes-derived), adapted for #[cube]
// This shows the STRUCTURE -- actual coefficients from libm source

#[cube]
pub fn erf_approx(x: f64) -> f64 {
    let abs_x = f64::abs(x);

    // Region 1: |x| < 0.84375 -- polynomial approximation
    // Region 2: 0.84375 <= |x| < 1.25 -- rational approx around erfc(1)
    // Region 3: 1.25 <= |x| < 2.857142 -- rational approx
    // Region 4: 2.857142 <= |x| < 6.0 -- rational approx
    // Region 5: |x| >= 6.0 -- erf(x) = 1.0 (within f64 precision)

    // Each region uses Horner's method polynomial evaluation
    // with pre-computed coefficients for numerator and denominator.
    // Use nested select() for region dispatch.

    let result = select(abs_x < 0.84375,
        erf_small(abs_x),
        select(abs_x < 1.25,
            erf_medium(abs_x),
            select(abs_x < 6.0,
                1.0 - erfc_large(abs_x),
                1.0
            )
        )
    );

    select(x < 0.0, -result, result)
}
```

### LDA_X Ext Params Handling
```rust
// Source: libxc-master/src/lda_x.c lines 33-47
// lda_x_params struct has one field: alpha (default 1.0)
// For XC_LDA_X (id=1), alpha is always 1.0
// For XC_LDA_C_XALPHA (id=6), alpha = 1.5 * ext_param[0] - 1.0

// In kernel, alpha is passed as scalar f64 argument:
#[cube(launch_unchecked)]
pub fn lda_x_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    alpha: f64,          // params->alpha, pre-computed by caller
    dens_threshold: f64,
    zeta_threshold: f64,
) { /* ... */ }
```

### Inline Test Pattern (Per D-07/D-08)
```rust
// Source: Design doc Section 7.5, CONTEXT.md D-07/D-08
#[cfg(test)]
mod tests {
    use super::*;
    use cubecl_cpu::{CpuDevice, CpuRuntime};

    // Helper to evaluate a #[cube] function on CubeCL CPU backend
    fn eval_cube_fn(/* setup */) -> f64 {
        let device = CpuDevice::default();
        let client = ComputeClient::load(&device);
        // Create input buffer, launch kernel, read result
        // ...
    }

    #[test]
    fn test_safe_cbrt_known_values() {
        assert_eq!(eval_cube_fn(safe_cbrt, 8.0), 2.0);
        assert_eq!(eval_cube_fn(safe_cbrt, -8.0), -2.0);  // SC-1
        assert_eq!(eval_cube_fn(safe_cbrt, 0.0), 0.0);
        assert_eq!(eval_cube_fn(safe_cbrt, 1.0), 1.0);
    }

    #[test]
    fn test_safe_cbrt_libm_sweep() {
        // Compare against libm::cbrt for 1000 points in [-100, 100]
        for i in 0..1000 {
            let x = -100.0 + 200.0 * (i as f64) / 999.0;
            let expected = libm::cbrt(x);
            let actual = eval_cube_fn(safe_cbrt, x);
            approx::assert_relative_eq!(actual, expected, max_relative = 1e-14);
        }
    }
}
```

## CubeCL API Reference (Verified)

### Available Math Traits on f64 in #[cube] Functions
| Trait | Method | Available | Source |
|-------|--------|-----------|--------|
| Abs | `f64::abs(x)` | YES | [VERIFIED: docs.rs/cubecl-core/0.9.0/frontend] |
| Sqrt | `f64::sqrt(x)` | YES | [VERIFIED: docs.rs/cubecl-core/0.9.0/frontend/trait.Sqrt] |
| Powf | `f64::powf(x, y)` | YES | [VERIFIED: docs.rs/cubecl-core/0.9.0/frontend/trait.Powf] |
| Powi | `f64::powi(x, n)` | YES | [VERIFIED: docs.rs/cubecl-core/0.9.0/frontend] |
| Exp | `f64::exp(x)` | YES | [VERIFIED: docs.rs/cubecl-core/0.9.0/frontend] |
| Log | `f64::log(x)` | YES | [VERIFIED: docs.rs/cubecl-core/0.9.0/frontend] |
| Sin/Cos/Tan | `f64::sin/cos/tan(x)` | YES | [VERIFIED: docs.rs/cubecl-core/0.9.0/frontend] |
| Erf | `f64::erf(x)` | YES (but precision unknown) | [VERIFIED: docs.rs/cubecl-core/0.9.0/frontend] |
| **Cbrt** | N/A | **NO** | [VERIFIED: not in frontend trait list] |
| select | `select(cond, a, b)` | YES | [VERIFIED: docs.rs/cubecl-core/0.9.0/frontend/fn.select] |
| ABSOLUTE_POS | Thread index | YES | [VERIFIED: vendored docs cubecl_3d_dft.md] |

### CubeCL Launch Pattern
```
1. ComputeClient::load(&CpuDevice::default())
2. client.create(bytemuck::cast_slice(&data))  -- upload
3. client.empty(n * size_of::<f64>())           -- allocate output
4. unsafe { kernel::launch_unchecked::<CpuRuntime>(&client, cube_count, cube_dim, args...) }
5. client.read_one(handle)                       -- download
6. f64::from_bytes(&bytes)                       -- convert
```

## LDA_X Canary Kernel Analysis

### Source File: `libxc-master/src/maple2c/lda_exc/lda_x.c`
- **Lines:** 1,434
- **Functions:** 10 (5 derivative orders x 2 spin modes)
- **Math dependencies:** `POW_1_3`, `my_piecewise3`, `M_CBRT3`, `M_CBRTPI`, `M_CBRT2`
- **Parameters:** `lda_x_params.alpha` (default 1.0 for XC_LDA_X, id=1)
- **Inputs:** `rho[0]` (unpol) or `rho[0], rho[1]` (pol)
- **Outputs:** zk (exc), vrho (vxc), v2rho2 (fxc), v3rho3 (kxc), v4rho4 (lxc)

### Function Complexity by Order
| Order | Unpol Temps | Pol Temps | Key Operations |
|-------|-------------|-----------|----------------|
| exc | ~16 | ~40 | POW_1_3, piecewise3 |
| vxc | +7 | +40 | Above + rho derivatives |
| fxc | +2 | +40 | Above + second derivatives |
| kxc | +3 | +40 | Above + third derivatives |
| lxc | +2 | +40 | Above + fourth derivatives |

### Polarized Kernel Key Differences
- Two density inputs: `rho[0]` (up), `rho[1]` (down)
- Total density: `t6 = rho[0] + rho[1]`
- Per-spin thresholding: `t1 = rho[0] <= dens_threshold`, `t26 = rho[1] <= dens_threshold`
- Uses `M_CBRT2` constant (2^(1/3)) for spin scaling
- Two output accumulations per derivative level (one per spin component)

### Metadata Issue (Noted)
The generated `src/meta/generated.rs` shows `XC_LDA_X` with `max_order: DerivativeOrder::Exc` and `ext_params: &[]`. The C source shows LDA_X supports through LXC (4th order) and has alpha parameter. This metadata incompleteness is from Phase 1's code generator -- it does not block Phase 2 because the canary kernel uses alpha as a direct scalar argument, and max_order is not checked during kernel evaluation in Phase 2. [VERIFIED: src/meta/generated.rs line 18, libxc-master/src/lda_x.c line 63]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Separate CPU + GPU kernels | Single #[cube] source for all backends | CubeCL 0.1.0 (2024) | Eliminates 649-functional duplication |
| CUDA/OpenCL raw kernels | CubeCL abstraction layer | CubeCL 0.9.0 (Jan 2026) | Write once, compile to CUDA/HIP/WGPU/CPU |
| Runtime erf library calls | Compile-time expanded polynomial approximation | Standard practice | Eliminates function call overhead on GPU |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | cubecl Cargo.toml feature flag structure: `cubecl = { version = "0.9.0", features = ["cpu"] }` | Standard Stack | LOW -- may need different feature syntax; verify on first `cargo build` |
| A2 | CubeCL `select()` is truly branchless on CPU backend | Architecture Patterns | LOW -- even if branchy on CPU, correctness unaffected; only GPU perf matters |
| A3 | `f64::from_bytes()` and `f64::as_bytes()` are available in CubeCL 0.9.0 | Code Examples | MEDIUM -- vendored docs show this but may be from different version; fallback is `bytemuck::cast_slice` |
| A4 | CubeCL 0.9.0 supports `<=` comparison returning bool inside #[cube] functions | Architecture Patterns | MEDIUM -- maple2c uses `<=` extensively; if CubeCL doesn't support it, need workaround |
| A5 | `calculate_cube_count_elemwise` utility exists in cubecl 0.9.0 | Don't Hand-Roll | LOW -- manual calculation is trivial if not available |
| A6 | CubeCL's built-in Erf for f64 may not meet 1e-15 precision target | Common Pitfalls | LOW risk -- we implement custom per D-05 regardless |

## Open Questions

1. **CubeCL `#[cube]` function calling other `#[cube]` functions**
   - What we know: The design doc says math core functions are inlined by CubeCL compiler. The vendored docs show standalone kernel functions.
   - What's unclear: Exact syntax for one `#[cube]` fn calling another. Is it `math::pow_1_3(x)` or does it need expansion syntax?
   - Recommendation: Test early with a simple `#[cube] fn a()` calling `#[cube] fn b()`. If it doesn't work, math functions may need to be macros or inline code.

2. **Horner polynomial evaluation with variable-length coefficients**
   - What we know: CubeCL `Array<f64>` exists for runtime-sized arrays. `#[comptime]` exists for compile-time values.
   - What's unclear: Can `poly_eval` accept a slice of coefficients inside a `#[cube]` function? Or must coefficient count be compile-time known?
   - Recommendation: Try `#[comptime]` const generics first. Fall back to fixed-size arrays with padding if needed. For erf specifically, coefficient counts are known at compile time.

3. **Bool type in CubeCL #[cube] functions**
   - What we know: `select(condition: bool, ...)` exists. The vendored docs use `if idx >= total { return; }`.
   - What's unclear: Whether comparison operators (`<=`, `<`, `>=`) return CubeCL bool or Rust bool inside `#[cube]` functions.
   - Recommendation: Test early. The maple2c translation depends heavily on `t2 = rho[0] / 2.0 <= dens_threshold` producing a boolean for `piecewise3`.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust toolchain | All compilation | YES | 1.92.0 (stable) | -- |
| cargo | Build system | YES | (bundled with rustc) | -- |
| cmake | verify/ oracle build | YES | (via cmake crate) | -- |
| libxc C source | Oracle comparison | YES | 7.0.0 vendored at libxc-master/ | -- |
| CUDA toolkit | Phase 7 only | N/A | -- | CubeCL CPU in Phase 2 |

**Missing dependencies with no fallback:** None -- all Phase 2 dependencies are available.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Built-in `#[test]` + approx 0.5.1 |
| Config file | None needed (Cargo test) |
| Quick run command | `cargo test -p libxc_rs --lib` |
| Full suite command | `cargo test --workspace` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| MATH-01 | safe_cbrt(-8) == -2 | unit | `cargo test -p libxc_rs math::powers::tests::test_safe_cbrt -x` | Wave 0 |
| MATH-02 | pow_1_3..pow_5_3 correct | unit | `cargo test -p libxc_rs math::powers -x` | Wave 0 |
| MATH-03 | piecewise3/5 branchless | unit | `cargo test -p libxc_rs math::piecewise -x` | Wave 0 |
| MATH-04 | erf/erfc f64 precision | unit + sweep | `cargo test -p libxc_rs math::erf -x` | Wave 0 |
| MATH-05 | Constants defined | unit | `cargo test -p libxc_rs math::constants -x` | Wave 0 |
| MATH-06 | Spin transforms | unit | `cargo test -p libxc_rs math::spin -x` | Wave 0 |
| MATH-07 | DFT quantities | unit | `cargo test -p libxc_rs math::dft_quantities -x` | Wave 0 |
| MATH-08 | Horner poly/rational eval | unit | `cargo test -p libxc_rs math::polynomials -x` | Wave 0 |
| MATH-09 | All math tested | integration | `cargo test -p libxc_rs math -x` | Wave 0 |
| MATH-10 | Cross-backend consistency | deferred | N/A -- Phase 7 | N/A |
| KERN-01 | Launch wrapper works | integration | `cargo test -p libxc_rs kernel::launch -x` | Wave 0 |
| KERN-02 | LDA_X oracle comparison | integration | `cargo test -p libxc_rs-verify lda_x_oracle -x` | Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p libxc_rs --lib`
- **Per wave merge:** `cargo test --workspace`
- **Phase gate:** Full suite green before /gsd-verify-work

### Wave 0 Gaps
- [ ] `src/math/powers.rs` tests -- covers MATH-01, MATH-02
- [ ] `src/math/piecewise.rs` tests -- covers MATH-03
- [ ] `src/math/erf.rs` tests -- covers MATH-04
- [ ] `src/math/constants.rs` tests -- covers MATH-05
- [ ] `src/math/spin.rs` tests -- covers MATH-06
- [ ] `src/math/dft_quantities.rs` tests -- covers MATH-07
- [ ] `src/math/polynomials.rs` tests -- covers MATH-08
- [ ] `verify/tests/lda_x_oracle.rs` -- covers KERN-02 (test file may already exist from Phase 1)
- [ ] Test helper to evaluate single `#[cube]` function via CubeCL CPU

## Security Domain

Security enforcement is not applicable to this phase. This is a numerical computation library with no network I/O, no user-facing input parsing, no authentication, and no data storage. All inputs are f64 arrays from calling DFT code. The only security-relevant concern is buffer bounds checking, which is handled by CubeCL's `ABSOLUTE_POS >= len` guards and Rust's slice bounds checking.

## Sources

### Primary (HIGH confidence)
- [docs.rs/cubecl-core/0.9.0/cubecl_core/frontend] - Verified math traits (Erf, Sqrt, Abs, Powf, Powi confirmed for f64; Cbrt confirmed absent)
- [docs.rs/cubecl-core/0.9.0/cubecl_core/frontend/fn.select] - select() signature and branchless semantics
- [docs.rs/cubecl-core/0.9.0/cubecl_core/frontend/trait.Powf] - Powf trait signature and f64 implementation
- [docs.rs/cubecl-core/0.9.0/cubecl_core/frontend/trait.Erf] - Erf trait exists, implemented for f64
- [docs.rs/crate/cubecl/latest] - CubeCL 0.9.0 confirmed as latest stable (Jan 15, 2026)
- docs/manual/Cubecl/cubecl_3d_dft.md - Vendored CubeCL f64 usage patterns, launch pattern, buffer management
- docs/manual/Cubecl/Cubecl_multi_compute.md - CubeCL multi-compute patterns
- libxc-master/src/maple2c/lda_exc/lda_x.c - LDA_X canary kernel source (1434 lines, 10 functions)
- libxc-master/src/util.h - Constants (M_CBRT3 etc.), macros (POW_1_3, my_piecewise3)
- libxc-master/src/lda_x.c - lda_x_params struct, alpha default value
- docs/design/libxc_rs_detailed_design.md Section 7 - Math core design specification
- docs/design/libxc_rs_detailed_design.md Section 12 - GPU/CubeCL design
- docs/design/libxc_rs_detailed_design.md Section 9.5/9.9 - Module responsibilities

### Secondary (MEDIUM confidence)
- [crates.io search: cubecl] - Version verification
- [crates.io search: libm] - libm 0.2.16 version verified
- Existing codebase: src/lib.rs, Cargo.toml, src/error/mod.rs, verify/build.rs, src/meta/generated.rs

### Tertiary (LOW confidence)
- [CubeCL Architecture Overview gist](https://gist.github.com/nihalpasham/570d4fe01b403985e1eaf620b6613774) - General CubeCL patterns

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - cubecl 0.9.0 version and trait availability verified via docs.rs
- Architecture: HIGH - patterns from design doc + vendored CubeCL examples + lda_x.c analysis
- Pitfalls: HIGH - cbrt/NaN issue is well-known; maple2c literal format verified from source
- CubeCL API details: MEDIUM - some specifics (#[cube] fn calling #[cube] fn, bool handling) need runtime verification

**Research date:** 2026-04-09
**Valid until:** 2026-05-09 (cubecl 0.9.0 is stable; no breaking changes expected within 30 days)
