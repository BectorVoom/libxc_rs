//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 85/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk85<F: Float>(t198: F, t196: F, rho1: F, sigma2: F, tau1: F) -> (F, F, F, F) {
    let t199 = sigma2 * t198;
    let t201 = 1.0 + 0.4e-2 * t199;
    let t202 = 1.0 / t201;
    let t207 = 1.0 / t196 / rho1;
    let t208 = tau1 * t207;
    (t199, t201, t202, t208)
}
