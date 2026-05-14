//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 608/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk608<F: Float>(t1153: F, t2873: F, t2869: F, t513: F, t1122: F, t2839: F, t509: F, t498: F, t1141: F, t521: F) -> (F, F, F, F, F, F, F) {
    let t2874 = t1153 * t2873;
    let t2877 = t513 * t2869;
    let t2880 = t1122 * t2873;
    let t2885 = t513 * t2873;
    let t2887 = t509 * t2839;
    let t2890 = t498 * t2869;
    let t2893 = 1.0 / t1141 / t521;
    (t2874, t2877, t2880, t2885, t2887, t2890, t2893)
}
