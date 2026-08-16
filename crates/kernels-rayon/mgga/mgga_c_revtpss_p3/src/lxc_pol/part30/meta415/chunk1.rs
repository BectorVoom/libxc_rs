//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1559/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1559(t15191: f64, t15197: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11339: f64, t11366: f64, t11368: f64, t11422: f64, t11423: f64, t15221: f64, t15230: f64) -> (f64, f64, f64) {
    let t15322 = 0.34431666666666666666e0_f64 * t15191;
    let t15324 = 0.13892666666666666667e0_f64 * t15197;
    let t15337 = -t11422 - t11423 + 0.6311625e0_f64 * t15221 + 0.23154444444444444444e-1_f64 * t11339 - 0.34431666666666666666e0_f64 * t11138 - 0.45908888888888888888e0_f64 * t11134 + 0.17215833333333333333e0_f64 * t11140 + 0.11477222222222222222e0_f64 * t11136 - 0.23154444444444444444e0_f64 * t11366 + 0.69463333333333333333e-1_f64 * t11368 + 0.3529725e1_f64 * t15230;
    (t15322, t15324, t15337)
}
