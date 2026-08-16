//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1287/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1287(t1469: f64, t2121: f64, t603: f64, t2247: f64, t30681: f64, t38: f64, t60673: f64, t7565: f64, t13272: f64, t29411: f64, t5842: f64, t60: f64) -> (f64, f64, f64, f64, f64) {
    let t111457 = t603 * t1469 * t2121;
    let t111516 = t2247 * t38 * t30681;
    let t111532 = t60673 * t7565;
    let t111537 = t13272 * t29411;
    let t111592 = t5842 * t60;
    (t111457, t111516, t111532, t111537, t111592)
}
