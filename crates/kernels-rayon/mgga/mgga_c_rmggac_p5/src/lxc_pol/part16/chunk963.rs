//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 963/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk963(t2061: f64, t9908: f64, t15093: f64, t9005: f64, t1704: f64, t325: f64, t2057: f64, t6376: f64, t645: f64, t797: f64, t6403: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46064 = t9908 * t2061;
    let t46066 = t15093 * t9005;
    let t46068 = t1704 * t325;
    let t46069 = t46068 * t2057;
    let t46075 = t645 * t6376;
    let t46076 = t797 * t46075;
    let t46083 = t649 * t6403;
    (t46064, t46066, t46068, t46069, t46075, t46076, t46083)
}
