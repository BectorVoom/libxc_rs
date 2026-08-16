//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 446/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk446(t2254: f64, t260: f64, t751: f64, t786: f64, t2299: f64, t154: f64, t2141: f64, t2320: f64, t2324: f64, t2331: f64, t2338: f64, t2342: f64, t2387: f64, t2389: f64, t2394: f64, t276: f64, t299: f64, t311: f64, t837: f64, t841: f64, t845: f64, t869: f64, t871: f64, t872: f64) -> f64 {
    let t2395 = t260 * t2254;
    let t2398 = t751 * t786;
    let t2401 = t260 * t2299;
    let t2404 = -0.47803703703703703703e-2_f64 * t154 * t2320 * t276 + 0.28682222222222222222e-1_f64 * t154 * t2324 * t276 - 0.28682222222222222222e-1_f64 * t154 * t837 * t845 + 0.21511666666666666667e-1_f64 * t154 * t2331 * t276 - 0.43023333333333333334e-1_f64 * t154 * t841 * t845 + 0.43023333333333333334e-1_f64 * t154 * t299 * t2338 - 0.21511666666666666667e-1_f64 * t154 * t299 * t2342 - t2387 * t260 + 2.0_f64 * t2389 * t872 - 2.0_f64 * t869 * t751 - 2.0_f64 * t2394 * t2395 + 2.0_f64 * t871 * t2398 + t871 * t2401 - t311 * t2141;
    t2404
}
