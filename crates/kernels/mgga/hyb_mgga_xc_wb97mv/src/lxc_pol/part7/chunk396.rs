//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 396/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk396<F: Float>(t1017: F, t1039: F, t1043: F, t1048: F, t1091: F, t1098: F, t1103: F, t1380: F, t1443: F, t1507: F, t1508: F, t458: F, t489: F, t198: F, tau1: F) -> (F, F) {
    let t1512 = t1017 + t1039 - t1043 - t1048 + t458 * t1508 + t1091 + 0.19751673498613801407e-1 * t1507 * t489 - t1098 - t1103 - t1380 - t1443;
    let t1514 = tau1 * t198;
    (t1512, t1514)
}
