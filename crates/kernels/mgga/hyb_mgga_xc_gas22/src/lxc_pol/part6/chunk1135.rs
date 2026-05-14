//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1135/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1135<F: Float>(t1166: F, t9526: F, t509: F, t515: F, t2867: F, t2874: F, t2828: F, t2938: F, t1143: F, t9653: F, t1184: F, t2987: F, t555: F, t2978: F, t6160: F, t2982: F) -> (F, F, F, F, F, F, F, F) {
    let t22754 = t1166 * t9526;
    let t22809 = t515 * t509;
    let t22858 = t2867 * t2874;
    let t22943 = t2938 * t2828;
    let t22954 = t1143 * t9653;
    let t22991 = t555 * t2987 * t1184;
    let t22994 = t555 * t6160 * t2978;
    let t22997 = t555 * t6160 * t2982;
    (t22754, t22809, t22858, t22943, t22954, t22991, t22994, t22997)
}
