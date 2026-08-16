//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2960/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2960(t12160: f64, t15688: f64, t1011: f64, t15689: f64, t15692: f64, t15693: f64, t15906: f64, t15907: f64, t16012: f64, t16081: f64, t16098: f64, t3117: f64, t42546: f64, t4915: f64, t4919: f64, t51869: f64, t51905: f64, t51998: f64, t53545: f64, t53885: f64, t53898: f64, t53901: f64, t53904: f64, t53909: f64) -> f64 {
    let t53914 = t12160 * t15688;
    let t53920 = -0.91464571985215438873e-2_f64 * t53885 * t16098 - t1011 * t4915 * t51869 / 144.0_f64 - 7.0_f64 / 54.0_f64 * t1011 * t16012 * t51998 + t1011 * t4919 * t51905 / 72.0_f64 - 0.42874018118069736972e-3_f64 * t53898 - 0.95275595817932748826e-4_f64 * t53901 + 0.28582678745379824648e-3_f64 * t42546 + 0.38586616306262763276e-2_f64 * t16081 * t3117 * t15907 * t53904 - 0.38586616306262763275e-2_f64 * t15906 * t3117 * t15907 * t53909 - 0.85748036236139473944e-3_f64 * t53914 * t15693 - 0.85748036236139473944e-3_f64 * t15689 * t53545 * t15692;
    t53920
}
