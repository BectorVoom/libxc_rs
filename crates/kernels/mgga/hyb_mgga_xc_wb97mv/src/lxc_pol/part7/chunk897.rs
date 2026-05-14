//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 897/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk897<F: Float>(t1153: F, t2952: F, t2839: F, t522: F, t1157: F, t2848: F, t1126: F, t2856: F, t1122: F, t2840: F, t2988: F, t638: F, t1173: F, t667: F, t544: F, t1877: F, t2990: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8025 = t2952 * t1153;
    let t8034 = t522 * t2839;
    let t8081 = t1157 * t2848;
    let t8089 = t1126 * t2856;
    let t8094 = t2840 * t1122;
    let t8126 = t2988 * t638;
    let t8130 = t667 * t1173;
    let t8131 = t8130 * t544;
    let t8135 = t2990 * t1877;
    (t8025, t8034, t8081, t8089, t8094, t8126, t8130, t8131, t8135)
}
