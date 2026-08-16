//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1074/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1074(t3357: f64, t3358: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t422: f64, t1130: f64, t1719: f64) -> (f64, f64, f64) {
    let t5060 = t3357 - 0.5936111111111111111e-2_f64 * t3358 - 0.5936111111111111111e-2_f64 * t5044 - 0.11872222222222222222e-1_f64 * t5049 + 0.35616666666666666666e-1_f64 * t5054 + 0.17808333333333333333e-1_f64 * t5058;
    let t5062 = 0.621814e-1_f64 * t5060 * t422;
    let t5063 = t1719 * t1130;
    (t5060, t5062, t5063)
}
