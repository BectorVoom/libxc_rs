//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1012/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1012(t3357: f64, t5044: f64, t6423: f64, t6427: f64, t6431: f64, t422: f64, t1733: f64, t5063: f64, t1732: f64, t1150: f64, t3384: f64, t1723: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6433 = t3357 - 0.11872222222222222222e-1_f64 * t5044 - 0.11872222222222222222e-1_f64 * t6423 + 0.35616666666666666666e-1_f64 * t6427 + 0.17808333333333333333e-1_f64 * t6431;
    let t6435 = 0.621814e-1_f64 * t6433 * t422;
    let t6437 = 2.0_f64 * t5063 * t1733;
    let t6438 = t1732 * t1732;
    let t6439 = t6438 * t1150;
    let t6441 = 2.0_f64 * t3384 * t6439;
    let t6442 = t1723 * t1723;
    (t6433, t6435, t6437, t6438, t6439, t6441, t6442)
}
