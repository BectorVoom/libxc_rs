//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 851/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk851<F: Float>(t2032: F, t339: F, t7189: F, t2589: F, t2597: F, t2565: F, t986: F, t2594: F, t982: F) -> (F, F, F, F, F, F) {
    let t7198 = 1.0 / t339 / t2032;
    let t7214 = 0.28842592592592592592e-1 * t7189;
    let t7221 = 0.53272592592592592592e-1 * t7189;
    let t7231 = t2589 * t2597;
    let t7249 = t2565 * t986;
    let t7254 = t982 * t2594;
    (t7198, t7214, t7221, t7231, t7249, t7254)
}
