//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 872/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk872<F: Float>(t479: F, t7591: F, t2693: F, t7532: F, t1062: F, t2745: F, t466: F, t1069: F, t2731: F, t2748: F, t474: F, t2663: F, t446: F, t437: F, t2667: F, t7584: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7592 = t479 * t7591;
    let t7593 = t7532 * t2693;
    let t7597 = 1.0 / t2745 / t1062;
    let t7598 = t466 * t7597;
    let t7599 = t2731 * t1069;
    let t7601 = 1.0 / t2748 / t474;
    let t7602 = t7599 * t7601;
    let t7606 = 1.0 / t2663 / t446;
    let t7607 = t437 * t7606;
    let t7608 = t7584 * t2667;
    (t7592, t7593, t7597, t7598, t7599, t7601, t7602, t7606, t7607, t7608)
}
