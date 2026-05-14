//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 615/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk615<F: Float>(t1143: F, t2922: F, t1128: F, t2869: F, t2873: F, t1142: F, t2895: F, t522: F, t2856: F, t516: F) -> (F, F, F, F, F, F) {
    let t2923 = t1143 * t2922;
    let t2928 = t1128 * t2869;
    let t2931 = t1128 * t2873;
    let t2934 = t2895 * t1142;
    let t2943 = t2922 * t522;
    let t2946 = t516 * t2856;
    (t2923, t2928, t2931, t2934, t2943, t2946)
}
