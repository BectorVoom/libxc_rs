//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1025/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1025(t10050: f64, t36612: f64, t46867: f64, t739: f64, t7577: f64, t40694: f64, t9222: f64, t2019: f64, t2020: f64, t9746: f64, t2010: f64, t2012: f64, t6492: f64) -> (f64, f64, f64, f64, f64) {
    let t47414 = t36612 * t10050;
    let t47417 = t739 * t7577 * t46867;
    let t47429 = t9222 * t40694;
    let t47432 = t2019 * t2020 * t9746;
    let t47435 = t2010 * t2012 * t6492;
    (t47414, t47417, t47429, t47432, t47435)
}
