//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 712/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk712(t3357: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t422: f64, t1126: f64, t1130: f64, t1151: f64, t1129: f64, t418: f64, t408: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3376 = t3357 - 0.11872222222222222222e-1_f64 * t3358 - 0.11872222222222222222e-1_f64 * t3365 + 0.35616666666666666666e-1_f64 * t3370 + 0.17808333333333333333e-1_f64 * t3374;
    let t3378 = 0.621814e-1_f64 * t3376 * t422;
    let t3379 = t1126 * t1130;
    let t3381 = 2.0_f64 * t3379 * t1151;
    let t3382 = t1129 * t418;
    let t3383 = 1.0_f64 / t3382;
    let t3384 = t408 * t3383;
    (t3376, t3378, t3379, t3381, t3382, t3383, t3384)
}
