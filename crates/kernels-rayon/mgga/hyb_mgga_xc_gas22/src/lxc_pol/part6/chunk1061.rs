//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1061/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1061(t1782: f64, t3804: f64, t1787: f64, t559: f64, t9909: f64, t555: f64, t558: f64, t6162: f64, t8159: f64, t8162: f64, t8183: f64, t8187: f64, t8199: f64, t8210: f64, t8216: f64, t8218: f64) -> (f64, f64, f64, f64) {
    let t10137 = t1782 * t3804;
    let t10141 = t1787 * t3804;
    let t10145 = t559 * t9909;
    let t10151 = -t555 * t558 * t10137 / 64.0_f64 - t555 * t558 * t10141 / 64.0_f64 - t555 * t558 * t10145 / 64.0_f64 - t8159 - t8162 + t6162 / 288.0_f64 - t8183 - t8187 / 48.0_f64 - t8199 - t8210 - t8216 - t8218;
    (t10137, t10141, t10145, t10151)
}
