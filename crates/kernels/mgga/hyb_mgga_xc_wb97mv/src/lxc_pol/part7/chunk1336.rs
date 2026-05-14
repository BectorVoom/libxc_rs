//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1336/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1336<F: Float>(t11977: F, t7833: F, t4077: F, t522: F, t2901: F, t11981: F, t11973: F, t1291: F, t9841: F, t9840: F, t396: F, t532: F, t1157: F, t2893: F, t4083: F, t9831: F) -> (F, F, F, F, F, F, F, F) {
    let t32639 = t7833 * t11977;
    let t32642 = t4077 * t522;
    let t32643 = t32642 * t2901;
    let t32655 = t11981 * t2901;
    let t32658 = t7833 * t11973;
    let t32669 = t9841 * t1291;
    let t32670 = t9840 * t32669;
    let t32673 = t532 * t396;
    let t32674 = t1157 * t32673;
    let t32675 = t4083 * t2893;
    let t32676 = t32675 * t9831;
    (t32639, t32643, t32655, t32658, t32670, t32674, t32675, t32676)
}
