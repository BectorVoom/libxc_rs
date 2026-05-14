//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 756/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk756<F: Float>(t424: F, t95: F, t401: F, t1299: F, t201: F, t1291: F, t212: F, tau1: F) -> (F, F, F, F, F) {
    let t4087 = t95 * t424;
    let t4088 = 1.0 / t401;
    let t4090 = 1.0 / t1299 / t201;
    let t4094 = t1291 * t212;
    let t4097 = tau1 * tau1;
    (t4087, t4088, t4090, t4094, t4097)
}
