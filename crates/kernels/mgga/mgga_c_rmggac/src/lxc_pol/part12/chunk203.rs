//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 203/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk203<F: Float>(t306: F, t84: F, t89: F, t312: F, t263: F, rho0: F, tau0: F) -> (F, F, F, F, F) {
    let t801 = t306 * rho0;
    let t803 = 1.0 / t84 / t801;
    let t804 = tau0 * t803;
    let t809 = 1.0 / t89;
    let t810 = t312 * t312;
    let t811 = t809 * t810;
    let t814 = 1.0 / t263;
    (t804, t809, t810, t811, t814)
}
