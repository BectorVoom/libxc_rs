//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1190/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1190<F: Float>(t1045: F, t3483: F, t260: F, t7259: F, t1390: F, t7375: F, t2473: F, t3491: F, t1403: F, t7376: F, t3530: F, t7405: F, t2224: F, t238: F, t3522: F, t3526: F) -> (F, F, F, F, F, F, F, F) {
    let t27052 = 8.0 * t3483 * t1045;
    let t27062 = t260 * t7259;
    let t27067 = t1390 * t7375;
    let t27070 = t3491 * t2473;
    let t27075 = t7376 * t1403;
    let t27111 = t3530 * t7405;
    let t27153 = t238 * t2224 * t3522;
    let t27156 = t238 * t2224 * t3526;
    (t27052, t27062, t27067, t27070, t27075, t27111, t27153, t27156)
}
