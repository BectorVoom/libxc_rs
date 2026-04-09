# Phase 4: Bulk Kernel Translation - Research

**Researched:** 2026-04-10
**Domain:** maple2c C-to-Rust kernel translation, CubeCL kernel authoring, oracle verification
**Confidence:** HIGH

## Summary

Phase 4 is the largest phase by volume: translating 262 maple2c C kernel files (42 LDA + 130 GGA + 90 MGGA + 4 special _vxc files) into Rust `#[cube]` functions. The canonical translation pattern was established in Phase 2 with LDA_X and is well-proven. The core challenge is scale (262 files, totaling ~4M lines of C) and increasing complexity from LDA (median 1,825 lines) through GGA (median 4,948 lines) to MGGA (median 21,261 lines, max 99,938 lines).

GGA kernels introduce the `sigma` (density gradient) input array and cross-derivative output fields (`vsigma`, `v2rhosigma`, `v2sigma2`, etc. -- 15 total output fields). MGGA kernels add `lapl` and `tau` inputs and expand to 70 output fields. The translation pattern remains identical: line-by-line C-to-Rust with exact variable name preservation and floating-point operation order fidelity. The verification oracle infrastructure must be extended from LDA-only to support GGA and MGGA C API calls (`xc_gga_exc_vxc_fxc_kxc`, `xc_gga_lxc`, `xc_mgga_exc_vxc_fxc_kxc`, `xc_mgga_lxc`).

**Primary recommendation:** Extend the existing LDA_X translation and verification patterns family-by-family (LDA -> GGA -> MGGA), adding missing math functions (`pow_3_2`, `pow_1_4`, `pow_7_3`) and oracle wrappers before starting each family batch. Test `mgga_c_rmggac` (100K lines) first among MGGAs to validate CubeCL compilation limits.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Fully manual hand-translation of each maple2c C file to Rust `#[cube]` functions, following the LDA_X pattern established in Phase 2 (D-12). No automated translator tool.
- **D-02:** One Rust file per functional, matching the 1:1 correspondence with maple2c source files (e.g., `lda_c_vwn.c` -> `lda_c_vwn.rs`). Each file contains all derivative order x spin mode combinations.
- **D-03:** Each functional gets its own launch wrapper file (matching the `launch_lda_x.rs` pattern from Phase 2).
- **D-04:** The 4 special `_vxc` files (`lda_xc_tih`, `gga_x_lb`, `mgga_x_2d_prp10`, `mgga_x_tb09`) are translated alongside their family batches, not deferred.
- **D-05:** Translate massive MGGA kernels (up to 100K lines) as-is, faithfully following the maple2c source. If CubeCL compilation fails or produces unacceptable compile times, split into sub-kernels per derivative order as a fallback.
- **D-06:** Test the largest MGGA kernel (`mgga_c_rmggac`, 100K lines) as the FIRST MGGA translation to surface compilation limit risks immediately.
- **D-07:** Per-family batch test files: `lda_oracle.rs`, `gga_oracle.rs`, `mgga_oracle.rs` in the verify/ crate.
- **D-08:** Each functional must pass oracle verification before moving to the next.
- **D-09:** Each derivative order tested independently.
- **D-10:** Tolerance tiers: energy (exc) <= 10^-12, VXC <= 10^-10, FXC <= 10^-8, KXC <= 10^-6, LXC <= 10^-4.
- **D-11:** Family order: LDA (42 files) -> GGA (130 files) -> MGGA (90 files).
- **D-12:** Dispatch wiring happens per-functional: each translated functional is immediately wired into the dispatch match statement and verified.

### Claude's Discretion
- Module structure under `kernel/gga/` and `kernel/mgga/` (flat vs grouped by sub-family)
- Whether to add a `kernel/mod.rs` re-export strategy or keep modules internal
- How to organize the per-family oracle test files (parametric test macros, test helper utilities)
- Whether the launch wrapper pattern needs adaptation for GGA/MGGA (additional input arrays: sigma, lapl, tau)
- Commit granularity during translation (per-functional vs small batches of related functionals)

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| KERN-03 | All LDA kernel files translated (~43 functionals) | 42 C files identified in `lda_exc/`, translation pattern from LDA_X established |
| KERN-04 | All GGA kernel files translated (~130 functionals) | 130 C files in `gga_exc/`, GGA adds sigma input + 15 output fields |
| KERN-05 | All MGGA kernel files translated (~80 functionals) | 90 C files in `mgga_exc/`, MGGA adds lapl/tau inputs + 70 output fields |
| KERN-06 | Kernel translations preserve floating-point operation order | Exact variable name + operation order preservation pattern proven with LDA_X |
| KERN-07 | Density thresholding: grid points below threshold skipped | Already implemented in LDA_X pattern: `rho[ip] / 2.0 <= dens_threshold` |
| KERN-08 | Output accumulation via += for mixed functional support | Already implemented in LDA_X pattern: `zk[ip] += tzk0` |
| KERN-09 | Each functional/order/spin combination is a separate kernel function | LDA_X has 10 functions (5 orders x 2 spins), same pattern applies |
| VERIFY-02 | All 649 functionals verified against oracle | This phase covers 262 kernel files; Phase 5 handles mixed/hybrid wiring for the rest |
| VERIFY-03 | Energy (exc): relative error <= 10^-12 | Tolerance tier verified with LDA_X oracle tests |
| VERIFY-04 | VXC: relative error <= 10^-10 | Tolerance tier from requirements |
| VERIFY-05 | FXC: relative error <= 10^-8 | Tolerance tier from requirements |
| VERIFY-06 | KXC: relative error <= 10^-6 | Tolerance tier from requirements |
| VERIFY-07 | LXC: relative error <= 10^-4 | Tolerance tier from requirements |
</phase_requirements>

## Project Constraints (from CLAUDE.md)

- **Tech stack**: Pure Rust + CubeCL 0.9.0; no C/Fortran in production path
- **Precision**: f64 only; energy relative error <= 10^-12 vs libxc oracle
- **Operation order**: Maple2c formula translations must preserve floating-point operation order for bit-level equivalence
- **Edition**: Rust 2024 (requires 1.85+), current toolchain is 1.92.0
- **Unsafe code**: Confined to compat/, kernel/launch.rs, and GPU buffer management

## Architecture Patterns

### Recommended Project Structure

```
src/kernel/
  launch.rs              # Shared launch infrastructure (existing)
  mod.rs                 # pub mod lda; pub mod gga; pub mod mgga;
  lda/
    mod.rs               # pub mod lda_x; pub mod launch_lda_x; pub mod lda_c_vwn; ...
    lda_x.rs             # Existing canary kernel (10 functions)
    launch_lda_x.rs      # Existing launch wrapper
    lda_c_vwn.rs         # New: kernel functions
    launch_lda_c_vwn.rs  # New: launch wrapper
    ... (42 kernel files + 42 launch files + 1 special _vxc)
  gga/
    mod.rs               # Flat: all 130+ modules listed
    gga_c_pbe.rs         # Kernel functions
    launch_gga_c_pbe.rs  # Launch wrapper
    ... (130 kernel files + 130 launch files + 1 special _vxc)
  mgga/
    mod.rs               # Flat: all 90+ modules listed
    mgga_x_tpss.rs       # Kernel functions
    launch_mgga_x_tpss.rs # Launch wrapper
    ... (90 kernel files + 90 launch files + 2 special _vxc)

src/eval/
  dispatch.rs            # Extend: dispatch_gga(), dispatch_mgga()
  mix.rs                 # Extend: evaluate_mixed_gga(), evaluate_mixed_mgga()

verify/
  src/lib.rs             # Extend: oracle_gga_all(), oracle_mgga_all()
  tests/
    lda_oracle.rs        # All 42 LDA functionals (new)
    gga_oracle.rs        # All 130 GGA functionals (new)
    mgga_oracle.rs       # All 90 MGGA functionals (new)
```

[VERIFIED: codebase grep] Module structure recommendation: flat layout under each family directory. With 130 GGA files, sub-grouping by prefix (gga_c_*, gga_x_*, hyb_gga_*) adds indirection without benefit -- the 1:1 mapping to maple2c source files is the organizing principle.

### Pattern 1: Kernel Translation (LDA -- established)

**What:** Each maple2c C file becomes one Rust file with N kernel functions (orders x spins).
**When to use:** Every functional translation.

LDA kernels take: `rho: &Array<f64>`, output arrays, scalar params.
```rust
// Source: src/kernel/lda/lda_x.rs (existing)
#[cube(launch_unchecked)]
pub fn lda_x_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // ... exact maple2c variable translation ...
        zk[ip] += tzk0;
    }
}
```

### Pattern 2: GGA Kernel Translation (new)

**What:** GGA kernels add `sigma` input and cross-derivative outputs.
**When to use:** All 130 GGA functionals.

GGA C function signature:
```c
// Source: libxc-master/src/maple2c/gga_exc/gga_c_pbe.c
func_exc_unpol(const xc_func_type *p, size_t ip, const double *rho,
               const double *sigma, xc_gga_out_params *out)
```

Rust translation adds sigma parameter and GGA-specific outputs:
```rust
// [ASSUMED] -- pattern extrapolated from LDA_X, needs validation on first GGA
#[cube(launch_unchecked)]
pub fn gga_c_pbe_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    // scalar params: functional-specific params + thresholds
    gamma: f64,
    beta: f64,
    bb: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // ... exact maple2c translation ...
        zk[ip] += tzk0;
        vrho[ip] += tvrho0;
        vsigma[ip] += tvsigma0;
    }
}
```

**Key differences from LDA:**
- Additional `sigma: &Array<f64>` input (unpolarized: 1 per point, polarized: 3 per point)
- Output arrays expand: VXC level adds `vsigma`; FXC adds `v2rhosigma`, `v2sigma2`; KXC adds `v3rho2sigma`, `v3rhosigma2`, `v3sigma3`; LXC adds 5 more
- Functional-specific params passed as scalars (69 of 130 GGA functionals use `params->`)
- Total output fields at LXC level: 15 (vs 5 for LDA)

### Pattern 3: MGGA Kernel Translation (new)

**What:** MGGA kernels add `lapl` and `tau` inputs and up to 70 output fields.
**When to use:** All 90 MGGA functionals.

MGGA C function signature:
```c
// Source: libxc-master/src/maple2c/mgga_exc/mgga_x_tpss.c
func_exc_unpol(const xc_func_type *p, size_t ip, const double *rho,
               const double *sigma, const double *lapl, const double *tau,
               xc_mgga_out_params *out)
```

**Key differences from GGA:**
- Additional `lapl: &Array<f64>` and `tau: &Array<f64>` inputs
- Output arrays expand massively: VXC adds `vlapl`, `vtau`; FXC adds 6 cross terms (rholapl, rhotau, sigmalapl, sigmatau, lapl2, lapltau, tau2); etc.
- Total output fields at LXC level: 70 (vs 15 for GGA)
- Conditional outputs: `vlapl` only written if `XC_FLAGS_NEEDS_LAPLACIAN` set; `vtau` only if `XC_FLAGS_NEEDS_TAU` -- this must be handled per-functional [VERIFIED: codebase analysis of mgga_x_tpss.c]

### Pattern 4: Launch Wrapper Adaptation for GGA/MGGA

**What:** Launch wrappers grow to handle more array arguments.
**When to use:** Every GGA and MGGA functional.

The existing `BufArg` pattern from `launch_lda_x.rs` scales directly -- just more arguments. GGA LXC-level wrappers will have ~18 BufArg parameters (rho, sigma, + 15 outputs + scalars). MGGA LXC-level wrappers will be very large (~75 parameters including all output arrays). [ASSUMED]

**Recommendation:** For MGGA, consider a struct-based approach for output buffer handles to keep function signatures manageable:
```rust
pub struct MggaOutputBufs<'a> {
    pub zk: &'a BufArg<'a>,
    pub vrho: &'a BufArg<'a>,
    pub vsigma: &'a BufArg<'a>,
    pub vlapl: Option<&'a BufArg<'a>>,
    pub vtau: Option<&'a BufArg<'a>>,
    // ... up to 70 fields
}
```

### Pattern 5: Dispatch Extension

**What:** `dispatch_gga()` and `dispatch_mgga()` functions mirroring `dispatch_lda()`.
**When to use:** Routing evaluation calls for GGA/MGGA functionals.

The existing `dispatch_lda` matches on `(order, spin)` to select the correct kernel. GGA and MGGA dispatch follows the same pattern but must also:
1. Upload sigma (GGA/MGGA) and lapl/tau (MGGA) input buffers
2. Create output handles for all applicable cross-derivative fields
3. Read back all populated output fields [VERIFIED: dispatch.rs pattern analysis]

### Pattern 6: Per-Functional External Parameters

**What:** Many functionals (18 LDA, 69 GGA, 49 MGGA = 136 total) use `params->` to access functional-specific parameters.
**When to use:** Any functional whose maple2c C code contains `params->field` references.

In the C code, `params` is a typed struct pointer cast from `p->params`. In the Rust translation, each parameter becomes a scalar argument to the `#[cube]` function. The dispatch layer must know which parameters each functional needs and pass them correctly.

**Example from gga_c_pbe.c:** Uses `params->gamma`, `params->BB`, `params->beta` -- these become three additional `f64` scalar arguments to the kernel function. [VERIFIED: codebase analysis]

### Anti-Patterns to Avoid

- **Reordering operations for "readability":** The maple2c temporaries (t1, t2, ...) define exact floating-point evaluation order. Reordering, combining, or "simplifying" expressions WILL change numerical results and break oracle verification. [VERIFIED: CLAUDE.md constraint]
- **Using generic `pow()` instead of specific power functions:** `pow(x, 1.0/3.0)` has different rounding from `cbrt(x)`. Always use the specific `pow_1_3()`, `pow_3_2()` etc. that match the C macro definitions. [VERIFIED: util.h analysis]
- **Batching translations without verification:** D-08 mandates each functional passes before proceeding. Do not translate 10 functionals then test them all.
- **Skipping the sigma threshold for GGA/MGGA:** GGA and MGGA kernels may need sigma thresholding (`sigma < sigma_threshold`) in addition to density thresholding. Check each kernel's C source for threshold patterns.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Power functions | Custom pow implementations | Existing `src/math/powers.rs` functions | Already verified against libm, CubeCL-compatible |
| Piecewise conditionals | if/else chains | `piecewise3()` / `piecewise5()` from `src/math/piecewise.rs` | Branch-free select semantics matching maple2c |
| Error functions | Manual polynomial approximation | `erf_approx()` / `erfc_approx()` from `src/math/erf.rs` | Already implemented and tested |
| Buffer management | Manual CubeCL buffer code | `kernel::launch` utilities | `create_input_buffer`, `create_zero_output_buffer`, `read_output_buffer` |
| Oracle FFI | Direct C calls | `verify/src/lib.rs` oracle functions | Safe wrappers with proper alloc/free lifecycle |

## Common Pitfalls

### Pitfall 1: Missing Power Function Implementations
**What goes wrong:** GGA/MGGA kernels use `POW_3_2`, `POW_1_4`, `POW_7_3`, `POW_2`, `POW_3` which are NOT yet implemented as `#[cube]` functions.
**Why it happens:** LDA_X only needed `pow_1_3`. The remaining power macros were not needed until bulk translation.
**How to avoid:** Implement all missing power functions before starting GGA translation:
- `pow_3_2(x)` = `x * sqrt(x)` -- used in 415 GGA + 709 MGGA occurrences
- `pow_1_4(x)` = `sqrt(sqrt(x))` -- 55 GGA + 230 MGGA occurrences
- `pow_7_3(x)` = `x * x * cbrt(x)` -- 1 file only (gga_xc_th2.c)
- `pow_2(x)` = `x * x` -- 135 GGA + 77 MGGA occurrences (inline `x * x` is also valid)
- `pow_3(x)` = `x * x * x` -- 15 MGGA occurrences
- `pow_5_3(x)` already exists in `powers.rs`
**Warning signs:** Compilation errors referencing undefined functions.
[VERIFIED: grep analysis of maple2c sources + existing powers.rs]

### Pitfall 2: GGA/MGGA Output Field Indexing in Polarized Mode
**What goes wrong:** Polarized GGA/MGGA kernels index output arrays with offsets like `ip*p->dim.vsigma + 0/1/2`. Getting the stride wrong corrupts all output.
**Why it happens:** Polarized sigma has 3 components (sigma_aa, sigma_ab, sigma_bb) per grid point. The C code indexes as `sigma[ip*3 + component]`. The Rust kernel must replicate this exactly.
**How to avoid:** Use the `Dimensions` struct (already implemented) to compute correct strides. For polarized GGA, `dims.vsigma = 3`, `dims.v2rhosigma = 6`, etc. Verify stride calculations match libxc for each output field.
**Warning signs:** Oracle verification failures only in polarized mode.
[VERIFIED: dims/mod.rs analysis, GGA C source indexing patterns]

### Pitfall 3: Conditional MGGA Output Fields (lapl/tau)
**What goes wrong:** Not all MGGA functionals use laplacian. The C code guards lapl outputs with `XC_FLAGS_NEEDS_LAPLACIAN`. Writing to lapl outputs for functionals that don't need them corrupts memory.
**Why it happens:** In C, the guard is a runtime flag check. In Rust, we need to know at translation time whether a functional uses laplacian.
**How to avoid:** Check the C source: if it writes to `out->vlapl`, the functional needs laplacian. If it does not, the kernel should not take lapl-related output arrays as parameters. This is a per-functional property discovered during translation.
**Warning signs:** NaN or zero in lapl-related outputs for functionals that do use laplacian.
[VERIFIED: mgga_x_tpss.c analysis showing conditional lapl/tau writes]

### Pitfall 4: External Parameter Mapping Errors
**What goes wrong:** 136 functionals use `params->field` where the field names and types vary per functional. Mapping wrong parameter values causes subtle numerical errors that may pass loose tolerance tiers but fail strict ones.
**Why it happens:** Each functional defines its own params struct in its C source header. The Rust dispatch must supply the correct default values.
**How to avoid:** For each functional with params, extract parameter names and defaults from the corresponding C header file (e.g., `gga_c_pbe.h` contains `gga_c_pbe_params` struct and default values). Pass these as scalar arguments to the kernel.
**Warning signs:** Energy values systematically wrong by a constant factor.
[VERIFIED: gga_c_pbe.c params analysis]

### Pitfall 5: CubeCL Compilation Time/Memory for Large Kernels
**What goes wrong:** The largest MGGA kernel (mgga_c_rmggac) is ~100K lines of C. The Rust translation will produce a massive `#[cube]` function that may exceed CubeCL's IR compilation capacity or take prohibitive compile times.
**Why it happens:** CubeCL compiles `#[cube]` functions through a proc-macro that generates IR at compile time. Very large functions produce very large IR.
**How to avoid:** D-06 mandates testing mgga_c_rmggac FIRST among MGGAs. If compilation fails, split into per-derivative-order sub-kernels (each order's function is self-contained in the C source). Median MGGA is 21K lines which should be fine; only the top ~10 files exceed 50K lines.
**Warning signs:** Cargo compile hangs, OOM during compilation, or rustc crashes.
[ASSUMED -- CubeCL IR limits not documented; risk assessment based on kernel sizes]

### Pitfall 6: Numeric Literal Translation Errors
**What goes wrong:** maple2c uses Maple-style float literals like `0.2e1` (= 2.0), `0.379785e1` (= 3.79785), `0.621814e-1` (= 0.0621814). Mistranslating these introduces systematic errors.
**Why it happens:** The Maple notation is unfamiliar. `0.2e1` looks like it should be `0.2` but is actually `2.0`.
**How to avoid:** Systematic conversion: `0.XYZeN` = `0.XYZ * 10^N`. The LDA_X translation (existing) demonstrates correct conversions. Verify each literal against the C source by computing the value.
**Warning signs:** Oracle verification failures with large relative errors (factor-of-10 errors).
[VERIFIED: lda_x.rs translation showing correct literal conversion]

### Pitfall 7: Dispatch Table Scaling
**What goes wrong:** With 262 functionals, each having up to 10 kernel variants, the dispatch match statement becomes enormous (~2620 arms).
**Why it happens:** The Phase 2 dispatch only handles LDA_X (1 functional). Scaling to 262 functionals with the same pattern creates an unmaintainable match block.
**How to avoid:** Use a two-level dispatch: first match on functional ID to select the correct launch module, then match on (order, spin) within that module. Each functional's dispatch logic stays in its own launch wrapper module, and the top-level dispatch is a functional ID -> handler routing.
**Warning signs:** Compilation times growing quadratically, match expression too large warnings.
[ASSUMED -- architectural recommendation for scaling]

## Code Examples

### GGA Oracle Function (to be added to verify/src/lib.rs)

```rust
// [ASSUMED] -- pattern extrapolated from existing oracle_lda_all
pub struct GgaOracleOutput {
    pub zk: Vec<f64>,
    pub vrho: Vec<f64>,
    pub vsigma: Vec<f64>,
    pub v2rho2: Vec<f64>,
    pub v2rhosigma: Vec<f64>,
    pub v2sigma2: Vec<f64>,
    pub v3rho3: Vec<f64>,
    pub v3rho2sigma: Vec<f64>,
    pub v3rhosigma2: Vec<f64>,
    pub v3sigma3: Vec<f64>,
    pub v4rho4: Vec<f64>,
    pub v4rho3sigma: Vec<f64>,
    pub v4rho2sigma2: Vec<f64>,
    pub v4rhosigma3: Vec<f64>,
    pub v4sigma4: Vec<f64>,
}

pub fn oracle_gga_all(func_id: i32, spin: i32, rho: &[f64], sigma: &[f64]) -> Result<GgaOracleOutput> {
    // ... similar to oracle_lda_all but calls xc_gga_exc_vxc_fxc_kxc + xc_gga_lxc
}
```

### Parametric Oracle Test Pattern (for batch verification)

```rust
// [ASSUMED] -- recommended test organization pattern
#[cfg(test)]
mod lda_oracle_tests {
    use super::*;

    struct FunctionalTestCase {
        id: i32,
        name: &'static str,
        max_order: u8,  // 3 for lda_c_pk09, 4 for most
        has_params: bool,
    }

    const LDA_FUNCTIONALS: &[FunctionalTestCase] = &[
        FunctionalTestCase { id: 1, name: "lda_x", max_order: 4, has_params: false },
        FunctionalTestCase { id: 7, name: "lda_c_vwn", max_order: 4, has_params: false },
        // ... all 42 LDA functionals
    ];

    #[test]
    fn test_all_lda_functionals_unpol() {
        for tc in LDA_FUNCTIONALS {
            for order in 0..=tc.max_order {
                // call oracle, call Rust kernel, compare at tolerance tier
            }
        }
    }
}
```

### Missing Power Function Implementations

```rust
// Source: libxc-master/src/util.h macro definitions
// These must be added to src/math/powers.rs

#[cube]
pub fn pow_3_2(x: f64) -> f64 {
    // POW_3_2(x) = (x)*sqrt(x)
    x * f64::sqrt(x)
}

#[cube]
pub fn pow_1_4(x: f64) -> f64 {
    // POW_1_4(x) = sqrt(sqrt(x))
    f64::sqrt(f64::sqrt(x))
}

#[cube]
pub fn pow_7_3(x: f64) -> f64 {
    // POW_7_3(x) = (x)*(x)*cbrt(x)
    x * x * safe_cbrt(x)
}

// POW_2(x) and POW_3(x) can be inlined as x*x and x*x*x respectively,
// but having named functions aids grep-ability
#[cube]
pub fn pow_2(x: f64) -> f64 {
    x * x
}

#[cube]
pub fn pow_3(x: f64) -> f64 {
    x * x * x
}
```
[VERIFIED: util.h macro definitions confirm these implementations]

## Kernel File Inventory

### File Counts (verified by directory listing)

| Family | _exc files | _vxc files | Total | With params |
|--------|-----------|------------|-------|-------------|
| LDA | 42 | 1 (lda_xc_tih) | 43 | 18 |
| GGA | 130 | 1 (gga_x_lb) | 131 | 69 |
| MGGA | 90 | 2 (mgga_x_2d_prp10, mgga_x_tb09) | 92 | 49 |
| **Total** | **262** | **4** | **266** | **136** |

[VERIFIED: `ls` and `grep` counts on libxc-master/src/maple2c/]

### Size Distribution (lines of C)

| Family | Min | Median | Mean | Max | Total |
|--------|-----|--------|------|-----|-------|
| LDA | 559 | 1,825 | 4,977 | 32,520 (lda_c_pk09) | 209K |
| GGA | 2,692 | 4,948 | 9,997 | 74,426 (gga_c_ft97) | 1.3M |
| MGGA | 7,108 | 21,261 | 27,086 | 99,938 (mgga_c_rmggac) | 2.4M |

[VERIFIED: `wc -l` analysis]

### Limited Derivative Order Functionals

| Functional | Max Order | Family |
|-----------|-----------|--------|
| lda_c_pk09 | 3 (no LXC) | LDA |
| mgga_c_b94 | 3 (no LXC) | MGGA |

All other 260 functionals support order 4 (LXC). [VERIFIED: grep for `maple2c_order`]

### Hybrid Functionals in Kernel Files

| Family | Count | Files |
|--------|-------|-------|
| LDA | 1 | hyb_lda_xc_bn05 |
| GGA | 3 | hyb_gga_* |
| MGGA | 6 | hyb_mgga_* |

These are kernel files for the XC component of hybrid functionals. The hybrid mixing itself (exact exchange fraction) is handled in Phase 5. [VERIFIED: ls hyb_*]

### Math Function Usage Across Kernels

| Function | GGA occurrences | MGGA occurrences | Implemented? |
|----------|----------------|-------------------|-------------|
| piecewise3 | 38,125 | 60,340 | Yes |
| piecewise5 | 7,505 | 6,784 | Yes |
| pow_1_3 | 6,624 | 5,609 | Yes |
| sqrt | 3,180 | 2,248 | Built-in |
| log | 1,951 | 2,177 | Built-in |
| exp | 1,295 | 1,489 | Built-in |
| M_CBRT3 | 1,210 | 780 | Yes |
| M_CBRT2 | 898 | 603 | Yes |
| pow_3_2 | 415 | 709 | **NO -- must add** |
| erf | 207 | 165 | Yes |
| atan | 170 | 15 | Built-in (f64::atan) |
| pow_2 | 135 | 77 | **NO -- must add** |
| fabs | 52 | 136 | Built-in (f64::abs) |
| pow_1_4 | 55 | 230 | **NO -- must add** |
| erfc | 35 | 0 | Yes |
| pow_5_3 | 10 | 0 | Yes |
| pow_3 | 0 | 15 | **NO -- must add** |
| pow_7_3 | 1 | 0 | **NO -- must add** |

[VERIFIED: grep -oh analysis of all maple2c files + powers.rs review]

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in `#[test]` + cargo test |
| Config file | Cargo.toml (workspace) |
| Quick run command | `cargo test -p libxc_rs --lib -- kernel::lda` |
| Full suite command | `cargo test -p libxc_rs-verify` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| KERN-03 | All LDA kernels pass oracle | integration | `cargo test -p libxc_rs-verify -- lda_oracle` | No -- Wave 0 |
| KERN-04 | All GGA kernels pass oracle | integration | `cargo test -p libxc_rs-verify -- gga_oracle` | No -- Wave 0 |
| KERN-05 | All MGGA kernels pass oracle | integration | `cargo test -p libxc_rs-verify -- mgga_oracle` | No -- Wave 0 |
| KERN-06 | FP operation order preserved | integration | Oracle comparison (implicit in KERN-03/04/05) | N/A |
| KERN-07 | Density thresholding correct | unit | `cargo test -p libxc_rs --lib -- kernel::lda` | Partial (LDA_X) |
| KERN-08 | Output += accumulation | unit | Existing dispatch tests | Yes |
| KERN-09 | Separate kernel per order/spin | unit | Compilation check (each function exists) | N/A |
| VERIFY-02 | All functionals verified | integration | All oracle tests pass | No -- Wave 0 |
| VERIFY-03 | exc <= 10^-12 | integration | Oracle exc tests | No -- Wave 0 |
| VERIFY-04 | VXC <= 10^-10 | integration | Oracle vxc tests | No -- Wave 0 |
| VERIFY-05 | FXC <= 10^-8 | integration | Oracle fxc tests | No -- Wave 0 |
| VERIFY-06 | KXC <= 10^-6 | integration | Oracle kxc tests | No -- Wave 0 |
| VERIFY-07 | LXC <= 10^-4 | integration | Oracle lxc tests | No -- Wave 0 |

### Sampling Rate
- **Per functional:** Run oracle test for that functional only
- **Per family batch:** Run full family oracle test
- **Phase gate:** `cargo test -p libxc_rs-verify` all green

### Wave 0 Gaps
- [ ] `verify/tests/lda_oracle.rs` -- batch oracle tests for all 42 LDA functionals
- [ ] `verify/tests/gga_oracle.rs` -- batch oracle tests for all 130 GGA functionals
- [ ] `verify/tests/mgga_oracle.rs` -- batch oracle tests for all 90 MGGA functionals
- [ ] `verify/src/lib.rs` -- `oracle_gga_all()` and `oracle_mgga_all()` functions
- [ ] `src/math/powers.rs` -- `pow_3_2`, `pow_1_4`, `pow_7_3`, `pow_2`, `pow_3`

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Single LDA_X kernel | All 262 kernels translated | This phase | Enables full functional coverage |
| LDA-only oracle | GGA + MGGA oracle wrappers | This phase | Enables verification of all families |
| LDA-only dispatch | Family-specific dispatch functions | This phase | Routes all evaluation calls correctly |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | GGA kernel pattern is a direct extension of LDA pattern (add sigma array) | Architecture Pattern 2 | LOW -- maple2c structure is consistent across families |
| A2 | MGGA launch wrappers may benefit from struct-based output parameter grouping | Architecture Pattern 4 | LOW -- cosmetic; flat parameters also work |
| A3 | CubeCL can handle 100K-line kernel compilation | Pitfall 5 | HIGH -- if wrong, must split kernels per derivative order |
| A4 | Two-level dispatch (ID -> handler -> order/spin) is better than flat match | Pitfall 7 | LOW -- either approach works, flat match may have longer compile times |
| A5 | `pow_2(x)` and `pow_3(x)` as named functions vs inline `x*x` | Code Examples | LOW -- both work, named functions aid consistency |

## Open Questions

1. **CubeCL compilation limits for large MGGA kernels**
   - What we know: The largest kernel is 100K lines of C. CubeCL proc-macro generates IR at compile time.
   - What's unclear: Maximum function size CubeCL can handle before OOM or timeout.
   - Recommendation: D-06 addresses this -- test mgga_c_rmggac first. If it fails, split per derivative order.

2. **External parameter default values**
   - What we know: 136 functionals use `params->field`. Default values are defined in C header files.
   - What's unclear: Where exactly each functional's default params are stored in the libxc source tree.
   - Recommendation: Check `libxc-master/src/<functional_name>.c` for the `xc_func_info_type` definition which includes the params struct and defaults. The oracle handles defaults automatically via `xc_func_init`.

3. **Dispatch architecture at scale**
   - What we know: Current dispatch is a single match on (order, spin) for one functional.
   - What's unclear: Best pattern for 262 functionals with varying parameter signatures.
   - Recommendation: Each functional's launch module exports a `dispatch_<name>()` function; the top-level dispatch routes by functional ID. This keeps each functional self-contained.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust toolchain | Compilation | Yes | 1.92.0 | -- |
| CubeCL CPU backend | Kernel execution | Yes | 0.9.0 | -- |
| libxc C source | Oracle reference | Yes | 7.0.0 (vendored) | -- |
| cmake | Building oracle | Yes | (via verify/build.rs) | -- |
| bindgen | Oracle FFI generation | Yes | 0.72.1 | -- |

**Missing dependencies with no fallback:** None
**Missing dependencies with fallback:** None

## Sources

### Primary (HIGH confidence)
- Codebase analysis: `src/kernel/lda/lda_x.rs`, `src/kernel/lda/launch_lda_x.rs`, `src/eval/dispatch.rs` -- established patterns
- Codebase analysis: `libxc-master/src/maple2c/{lda,gga,mgga}_exc/` -- translation source files
- Codebase analysis: `libxc-master/src/util.h` -- macro definitions (POW_*, piecewise*, constants)
- Codebase analysis: `src/math/powers.rs`, `src/math/piecewise.rs`, `src/math/erf.rs` -- existing math functions
- Codebase analysis: `verify/src/lib.rs` -- existing oracle infrastructure
- Codebase analysis: `src/dims/mod.rs` -- dimension calculations for all families/spins
- Codebase analysis: `src/output/mod.rs` -- GgaOutput (15 fields), MggaOutput (70 fields) already defined

### Secondary (MEDIUM confidence)
- `libxc-master/src/xc.h` -- C API function signatures for GGA/MGGA evaluation

### Tertiary (LOW confidence)
- CubeCL compilation limits for very large functions -- no documentation found

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all dependencies already in place from prior phases
- Architecture: HIGH -- patterns established by LDA_X, straightforward extension
- Pitfalls: HIGH -- identified through systematic codebase analysis
- CubeCL large kernel handling: LOW -- untested assumption (A3)

**Research date:** 2026-04-10
**Valid until:** 2026-05-10 (stable domain, no external dependency changes expected)
