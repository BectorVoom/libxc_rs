//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2889/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2889(t17279: f64, t699: f64, t17240: f64, t17243: f64, t136: f64, t2826: f64, t59715: f64, t10304: f64, t59751: f64, t59719: f64, t59706: f64, t41880: f64, t59711: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t60308 = t699 * t17279;
    let t60310 = t699 * t17240;
    let t60312 = t699 * t17243;
    let t60315 = t136 * t2826 * t59715;
    let t60318 = t136 * t10304 * t59751;
    let t60321 = t136 * t2826 * t59719;
    let t60324 = t136 * t10304 * t59706;
    let t60327 = t136 * t41880 * t59711;
    (t60308, t60310, t60312, t60315, t60318, t60321, t60324, t60327)
}
