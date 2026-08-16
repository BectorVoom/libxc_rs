//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3122/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3122(t15690: f64, t3153: f64, t372: f64, t11921: f64, t15716: f64, t15717: f64, t247: f64, t1041: f64, t1670: f64, t42994: f64, t11922: f64, t15786: f64, t4892: f64) -> (f64, f64, f64, f64) {
    let t55209 = t372 * t15690 * t3153;
    let t55233 = t15716 * t247 * t11921 * t15717;
    let t55247 = t1041 * t42994 * t1670;
    let t55265 = t4892 * t11922 * t15786;
    (t55209, t55233, t55247, t55265)
}
