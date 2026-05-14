//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 638/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk638<F: Float>(t1924: F, t615: F, t1205: F, t81: F, t1937: F) -> (F, F, F, F, F) {
    let t3089 = t1924 * t615;
    let t3090 = t81 * t1205;
    let t3093 = t1205 * t615;
    let t3103 = t1937 * t1205;
    let t3104 = t81 * t615;
    (t3089, t3090, t3093, t3103, t3104)
}
