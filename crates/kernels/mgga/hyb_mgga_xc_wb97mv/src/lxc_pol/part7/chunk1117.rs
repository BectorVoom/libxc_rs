//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1117/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1117<F: Float>(t4554: F, t522: F, t1137: F, t2818: F, t4558: F, t2824: F, t1106: F, t4545: F, t1514: F, t1523: F, t1520: F, t513: F, t5427: F, t1114: F, t4550: F, t1128: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12017 = t522 * t4554;
    let t12018 = t1137 * t12017;
    let t12021 = t4558 * t2818;
    let t12024 = t4558 * t2824;
    let t12029 = t1106 * t4545;
    let t12038 = t1514 * t1523;
    let t12041 = t1520 * t513;
    let t12046 = t5427 * t513;
    let t12049 = t4550 * t1114;
    let t12050 = t1128 * t12049;
    (t12017, t12018, t12021, t12024, t12029, t12038, t12041, t12046, t12049, t12050)
}
