//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 162/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk162<F: Float>(t115: F, t119: F, t208: F) -> (F, F, F) {
    let t501 = 3.0 / 10.0 * t115 * (t119 + t208);
    let t503 = 2.0 * t119 * t208;
    let t504 = t501 - t503;
    (t501, t503, t504)
}
