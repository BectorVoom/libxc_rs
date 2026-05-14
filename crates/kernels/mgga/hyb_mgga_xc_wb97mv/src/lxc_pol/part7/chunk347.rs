//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 347/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk347<F: Float>(t43: F, t1187: F, t587: F, t591: F, t595: F, t599: F, t603: F, t607: F, t611: F, t1186: F) -> (F, F, F, F, F, F, F, F) {
    let t45 = 0.135e1 < t43;
    let t1190 = t587 * t1187;
    let t1192 = t591 * t1187;
    let t1194 = t595 * t1187;
    let t1196 = t599 * t1187;
    let t1198 = t603 * t1187;
    let t1200 = t607 * t1187;
    let t1202 = t611 * t1187;
    let t1205 = piecewise3(t45, 0.0, t1186);
    (t1190, t1192, t1194, t1196, t1198, t1200, t1202, t1205)
}
