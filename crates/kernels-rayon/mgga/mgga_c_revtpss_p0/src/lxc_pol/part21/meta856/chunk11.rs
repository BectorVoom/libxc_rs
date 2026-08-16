//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3258/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3258(t1913: f64, t4168: f64, t18217: f64, t571: f64, t1921: f64, t4153: f64, t1464: f64, t5789: f64, t18177: f64, t575: f64, t13226: f64, t13250: f64, t1456: f64, t1458: f64, t18178: f64, t1914: f64, t3: f64, t39397: f64, t39399: f64, t39401: f64, t39403: f64, t4154: f64, t47730: f64, t5790: f64, t5808: f64, t60560: f64, t60599: f64) -> f64 {
    let t60607 = t1913 * t4168;
    let t60609 = t571 * t18217;
    let t60611 = t4153 * t1921;
    let t60616 = t5789 * t1464;
    let t60618 = t18177 * t575;
    let tv4rho41 = t3 * t575 * t60560 + t13226 * t1921 + t13250 * t1914 + 3.0_f64 * t1456 * t18217 + t1458 * t60599 + 3.0_f64 * t1464 * t18178 + 3.0_f64 * t4154 * t5808 + 3.0_f64 * t4168 * t5790 + t39397 + 3.0_f64 * t39399 + 3.0_f64 * t39401 + t39403 + 6.0_f64 * t47730 + 3.0_f64 * t60607 + 3.0_f64 * t60609 + 3.0_f64 * t60611 + 6.0_f64 * t60616 + 3.0_f64 * t60618;
    tv4rho41
}
