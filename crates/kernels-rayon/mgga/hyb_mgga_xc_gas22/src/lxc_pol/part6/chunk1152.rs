//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1152/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1152(t1129: f64, t1297: f64, t1117: f64, t1128: f64, t2880: f64, t510: f64, t2903: f64, t521: f64, t1134: f64, t1139: f64, t2874: f64, t518: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13687 = t1297 * t1129;
    let t14626 = t1117 * t1128;
    let t14635 = t510 * t2880;
    let t14638 = t2903 * t521;
    let t14641 = t1134 * t1139;
    let t14648 = t518 * t2874;
    (t13687, t14626, t14635, t14638, t14641, t14648)
}
