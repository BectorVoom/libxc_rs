//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1206/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1206(t2867: f64, t2874: f64, t2828: f64, t2938: f64, t1143: f64, t9653: f64, t1184: f64, t2987: f64, t555: f64, t2978: f64, t6160: f64, t2982: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22858 = t2867 * t2874;
    let t22943 = t2938 * t2828;
    let t22954 = t1143 * t9653;
    let t22991 = t555 * t2987 * t1184;
    let t22994 = t555 * t6160 * t2978;
    let t22997 = t555 * t6160 * t2982;
    (t22858, t22943, t22954, t22991, t22994, t22997)
}
