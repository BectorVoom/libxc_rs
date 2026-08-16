//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1017/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1017(t1123: f64, t1530: f64, t1118: f64, t1159: f64, t7642: f64, t536: f64, t2824: f64, t3701: f64, t17: f64, t2880: f64, t531: f64, t1129: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9522 = t1530 * t1123;
    let t9523 = t1118 * t9522;
    let t9526 = t7642 * t1159;
    let t9527 = t536 * t9526;
    let t9528 = t3701 * t2824;
    let t9531 = t2880 * t17;
    let t9532 = t9531 * t531;
    let t9533 = t536 * t9532;
    let t9534 = t1530 * t1129;
    (t9522, t9523, t9526, t9527, t9528, t9531, t9532, t9533, t9534)
}
