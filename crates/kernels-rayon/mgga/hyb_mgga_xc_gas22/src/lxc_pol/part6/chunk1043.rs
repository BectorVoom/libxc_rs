//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1043/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1043(t1117: f64, t2880: f64, t1123: f64, t3701: f64, t1139: f64, t2903: f64, t1129: f64, t3727: f64, t1134: f64, t2874: f64, t1539: f64, t2893: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9765 = t1117 * t2880;
    let t9766 = t3701 * t1123;
    let t9769 = t2903 * t1139;
    let t9770 = t3727 * t1129;
    let t9773 = t1134 * t2874;
    let t9778 = t1539 * t2893;
    (t9765, t9766, t9769, t9770, t9773, t9778)
}
