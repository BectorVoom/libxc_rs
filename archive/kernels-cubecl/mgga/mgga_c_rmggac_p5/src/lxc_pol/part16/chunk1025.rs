//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1025/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1025<F: Float>(t10050: F, t36612: F, t46867: F, t739: F, t7577: F, t40694: F, t9222: F, t2019: F, t2020: F, t9746: F, t2010: F, t2012: F, t6492: F) -> (F, F, F, F, F) {
    let t47414 = t36612 * t10050;
    let t47417 = t739 * t7577 * t46867;
    let t47429 = t9222 * t40694;
    let t47432 = t2019 * t2020 * t9746;
    let t47435 = t2010 * t2012 * t6492;
    (t47414, t47417, t47429, t47432, t47435)
}
