//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 495/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk495(t3356: f64, t1178: f64, t444: f64, t439: f64, t3413: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3483 = 0.12361111111111111111e-1_f64 * t3356;
    let t3494 = t1178 * t444;
    let t3495 = 1.0_f64 / t3494;
    let t3496 = t439 * t3495;
    let t3503 = 0.40256666666666666667e0_f64 * t3356;
    let t3510 = 0.137975e0_f64 * t3413;
    let t3519 = t1178 * t1178;
    let t3520 = 1.0_f64 / t3519;
    (t3483, t3495, t3496, t3503, t3510, t3519, t3520)
}
