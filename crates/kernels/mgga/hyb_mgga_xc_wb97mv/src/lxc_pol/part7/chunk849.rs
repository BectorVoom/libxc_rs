//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 849/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk849<F: Float>(t6759: F, t2293: F, t260: F, t222: F, t341: F, t6129: F, t1847: F, t929: F) -> (F, F, F, F, F, F) {
    let t6986 = 0.28842592592592592592e-1 * t6759;
    let t6993 = 0.53272592592592592592e-1 * t6759;
    let t7034 = t260 * t2293;
    let t7189 = t222 * t6129 * t341;
    let t7190 = 0.55403703703703703703e-1 * t7189;
    let t7192 = t222 * t1847 * t929;
    (t6986, t6993, t7034, t7189, t7190, t7192)
}
