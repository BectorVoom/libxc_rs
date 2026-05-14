//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1200/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1200<F: Float>(t10034: F, t3803: F, t1126: F, t16106: F, t1157: F, t16063: F, t1122: F, t9988: F, t2893: F, t535: F, t7817: F, t10185: F, t2952: F, t516: F, t7837: F, t536: F, t8020: F) -> (F, F, F, F, F, F, F, F) {
    let t28117 = t3803 * t10034;
    let t28161 = t1126 * t16106;
    let t28165 = t1157 * t16063;
    let t28248 = t9988 * t1122;
    let t28338 = t535 * t7817 * t2893;
    let t28342 = t2952 * t10185;
    let t28347 = t516 * t7837 * t2893;
    let t28351 = t1157 * t536 * t8020;
    (t28117, t28161, t28165, t28248, t28338, t28342, t28347, t28351)
}
