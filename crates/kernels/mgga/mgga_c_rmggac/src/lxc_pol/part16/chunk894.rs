//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 894/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk894<F: Float>(t1550: F, t6522: F, t7778: F, t1990: F, t9826: F, t6355: F, t9005: F, t11905: F, t2301: F, t10050: F, t36612: F, t46867: F, t739: F, t7577: F, t40694: F, t9222: F) -> (F, F, F, F, F, F, F) {
    let t47393 = t1550 * t7778 * t6522;
    let t47405 = t9826 * t1990;
    let t47408 = t6355 * t9005;
    let t47410 = t11905 * t2301;
    let t47414 = t36612 * t10050;
    let t47417 = t739 * t7577 * t46867;
    let t47429 = t9222 * t40694;
    (t47393, t47405, t47408, t47410, t47414, t47417, t47429)
}
