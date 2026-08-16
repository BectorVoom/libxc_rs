//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 846/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk846(t3243: f64, t3297: f64, t136: f64, t1113: f64, t3248: f64, t3252: f64, t3238: f64, t3245: f64, t3250: f64, t3254: f64, t3272: f64, t3280: f64, t3282: f64, t3288: f64, t3290: f64, t3294: f64, t3295: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3298 = t3297 * t3243;
    let t3299 = t136 * t3298;
    let t3301 = t1113 * t3248;
    let t3302 = t136 * t3301;
    let t3304 = t1113 * t3252;
    let t3305 = t136 * t3304;
    let t3307 = -0.9494625e0_f64 * t3272 + 0.1898925e1_f64 * t3280 + t3282 - 0.19931111111111111111e0_f64 * t3238 - 0.19931111111111111111e0_f64 * t3245 + 0.59793333333333333334e0_f64 * t3250 + 0.29896666666666666667e0_f64 * t3254 + 0.15358125e0_f64 * t3288 + 0.3071625e0_f64 * t3290 + t3294 - 0.10954222222222222222e0_f64 * t3295 - 0.27385555555555555556e-1_f64 * t3299 + 0.16431333333333333333e0_f64 * t3302 + 0.82156666666666666667e-1_f64 * t3305;
    (t3298, t3299, t3301, t3302, t3304, t3305, t3307)
}
