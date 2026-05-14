//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1192/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1192<F: Float>(t2572: F, t3574: F, t2597: F, t9385: F, t3563: F, t7362: F, t3596: F, t7318: F, t9411: F, t967: F, t1415: F, t7360: F, t2555: F, t3541: F, t1408: F, t7359: F) -> (F, F, F, F, F, F, F, F) {
    let t27399 = t3574 * t2572;
    let t27450 = t9385 * t2597;
    let t27474 = t3563 * t7362;
    let t27488 = t3596 * t7318;
    let t27538 = t9411 * t967;
    let t27545 = t7360 * t1415;
    let t27550 = t3541 * t2555;
    let t27555 = t1408 * t7359;
    (t27399, t27450, t27474, t27488, t27538, t27545, t27550, t27555)
}
