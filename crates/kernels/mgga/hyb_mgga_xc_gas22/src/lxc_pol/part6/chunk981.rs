//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 981/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk981<F: Float>(t1123: F, t1297: F, t1129: F, t4489: F, t17: F, t2874: F, t531: F, t524: F, t1157: F, t7744: F, t3748: F, t9503: F, t1171: F, t536: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9566 = t1297 * t1123;
    let t9567 = t9566 * t1129;
    let t9568 = t4489 * t9567;
    let t9573 = t2874 * t17;
    let t9574 = t9573 * t531;
    let t9575 = t524 * t9574;
    let t9586 = t1157 * t7744;
    let t9587 = t524 * t9586;
    let t9588 = t3748 * t9503;
    let t9593 = t1171 * t7744;
    let t9594 = t536 * t9593;
    (t9566, t9567, t9568, t9573, t9574, t9575, t9586, t9587, t9588, t9593, t9594)
}
