//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1149/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1149(t1123: f64, t4576: f64, t1129: f64, t3663: f64, t4851: f64, t1134: f64, t3760: f64, t3767: f64, t518: f64, t1117: f64, t3771: f64, t3701: f64, t9625: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11575 = t4576 * t1123;
    let t11578 = t4576 * t1129;
    let t11583 = t3663 * t4851;
    let t11586 = t1134 * t3760;
    let t11589 = t518 * t3767;
    let t11594 = t1117 * t3771;
    let t11597 = t9625 * t3701;
    (t11575, t11578, t11583, t11586, t11589, t11594, t11597)
}
