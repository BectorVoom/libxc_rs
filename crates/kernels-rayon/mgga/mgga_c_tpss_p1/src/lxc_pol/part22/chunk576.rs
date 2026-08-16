//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 576/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk576(t2454: f64, t2455: f64, t2462: f64, t2467: f64, t2471: f64, t285: f64, t841: f64, t845: f64, t867: f64, t281: f64, t844: f64, t269: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2473 = t2454 + 0.11872222222222222222e-1_f64 * t2455 - 0.11872222222222222222e-1_f64 * t2462 + 0.35616666666666666666e-1_f64 * t2467 - 0.17808333333333333333e-1_f64 * t2471;
    let t2475 = 0.621814e-1_f64 * t2473 * t285;
    let t2476 = t841 * t845;
    let t2478 = 2.0_f64 * t2476 * t867;
    let t2479 = t844 * t281;
    let t2480 = 1.0_f64 / t2479;
    let t2481 = t269 * t2480;
    (t2473, t2475, t2476, t2478, t2480, t2481)
}
