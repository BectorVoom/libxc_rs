//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1185/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1185<F: Float>(t1352: F, t6903: F, t1364: F, t6966: F, t2317: F, t6968: F, t2323: F, t6876: F, t838: F, t9022: F, t6982: F, t3369: F, t6862: F, t1340: F, t6919: F, t2325: F, t9000: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26610 = t6903 * t1352;
    let t26616 = t6966 * t1364;
    let t26617 = t6968 * t2317;
    let t26621 = t2323 * t1364;
    let t26624 = t6876 * t1352;
    let t26634 = t9022 * t838;
    let t26637 = t6982 * t1364;
    let t26666 = t3369 * t6862;
    let t26706 = t6919 * t1340;
    let t26745 = t9000 * t2325;
    (t26610, t26616, t26617, t26621, t26624, t26634, t26637, t26666, t26706, t26745)
}
