//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 865/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk865<F: Float>(t457: F, t7500: F, t2640: F, t6811: F, t2639: F, t16: F, t3038: F, t1022: F, t15: F, t221: F, t435: F, t12: F, t2651: F, t1029: F, t14: F, t237: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7501 = t7500 * t457;
    let t7503 = t2640 * t6811;
    let t7504 = t2639 * t7503;
    let t7506 = t16 * t3038;
    let t7507 = t1022 * t7506;
    let t7509 = t15 * t3038;
    let t7510 = t221 * t7509;
    let t7512 = 1.0/pow_3_2(t435);
    let t7513 = t7512 * t12;
    let t7514 = t7513 * t457;
    let t7516 = t2651 * t7503;
    let t7518 = t1029 * t7506;
    let t7521 = t237 * t14 * t6811;
    (t7501, t7504, t7507, t7509, t7510, t7513, t7514, t7516, t7518, t7521)
}
