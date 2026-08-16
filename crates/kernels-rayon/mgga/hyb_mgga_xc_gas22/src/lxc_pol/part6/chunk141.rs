//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 141/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk141(t419: f64, t423: f64, t407: f64, t328: f64, t409: f64, t198: f64, t212: f64, t398: f64, t401: f64, t405: f64, t408: f64, t410: f64, t414: f64, t415: f64) -> (f64, f64, f64, f64, f64) {
    let t424 = t419 * t423;
    let t427 = t407 * t407;
    let t428 = t328 * t427;
    let t429 = t409 * t409;
    let t430 = 1.0_f64 / t429;
    let t431 = t428 * t430;
    let t436 = 0.46914023462026644e0_f64 * t398 * t198 * t401 + t405 * t212 + t408 * t410 + 0.10661445329398457901e-1_f64 * t415 * t424 + 0.10661445329398457901e-1_f64 * t431 * t414 * t419 * t423;
    (t428, t429, t430, t431, t436)
}
