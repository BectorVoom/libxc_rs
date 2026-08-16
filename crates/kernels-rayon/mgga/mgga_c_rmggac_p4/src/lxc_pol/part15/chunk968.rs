//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 968/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk968(t16043: f64, t9964: f64, t30344: f64, t3351: f64, t3352: f64, t515: f64, t17787: f64, t9005: f64, t10112: f64, t325: f64, t2057: f64, t12970: f64, t2061: f64) -> (f64, f64, f64, f64, f64) {
    let t46034 = t16043 * t9964;
    let t46038 = t3351 * t3352 * t515 * t30344;
    let t46040 = t17787 * t9005;
    let t46042 = t10112 * t325;
    let t46043 = t46042 * t2057;
    let t46045 = t12970 * t2061;
    (t46034, t46038, t46040, t46043, t46045)
}
