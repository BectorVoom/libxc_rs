//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1121/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1121<F: Float>(t1142: F, t4083: F, t2848: F, t529: F, t1122: F, t505: F, t1117: F, t2856: F, t511: F, t2860: F, t532: F, t1148: F, t1153: F, t536: F, t7926: F, t517: F, t7907: F, tau0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14633 = t4083 * t1142;
    let t15321 = t1142 * tau0;
    let t15553 = t529 * t2848;
    let t15560 = t505 * t1122;
    let t15563 = t1117 * t1122;
    let t15570 = t511 * t2856;
    let t15599 = t2860 * t532;
    let t15602 = t1148 * t1153;
    let t16026 = t7926 * t536;
    let t16063 = t7907 * t517;
    (t14633, t15321, t15553, t15560, t15563, t15570, t15599, t15602, t16026, t16063)
}
