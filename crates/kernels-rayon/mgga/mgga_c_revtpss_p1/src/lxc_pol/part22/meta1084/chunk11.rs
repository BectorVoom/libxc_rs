//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3936/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3936(t22571: f64, t571: f64, t1458: f64, t18178: f64, t18217: f64, t1914: f64, t1921: f64, t4168: f64, t5790: f64, t5808: f64, t60609: f64, t60611: f64, t60616: f64, t60618: f64, t6937: f64, t75727: f64, t75760: f64, t75792: f64) -> f64 {
    let t75796 = t571 * t22571;
    let t75801 = 2.0_f64 * t60609 + 2.0_f64 * t18178 * t1921 + 2.0_f64 * t60611 + 2.0_f64 * t75727 + 4.0_f64 * t60616 + 4.0_f64 * t5790 * t5808 + t1458 * (t75760 + t75792) + 2.0_f64 * t60618 + 2.0_f64 * t75796 + t6937 * t4168 + 2.0_f64 * t1914 * t18217;
    t75801
}
