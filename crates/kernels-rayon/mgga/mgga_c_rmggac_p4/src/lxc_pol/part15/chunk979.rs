//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 979/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk979(t2115: f64, t46129: f64, t2118: f64, t46177: f64, t46181: f64, t7633: f64, t46185: f64, t7641: f64, t46116: f64, t851: f64, t46121: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46189 = t2115 * t46129;
    let t46191 = t2118 * t46177;
    let t46193 = t7633 * t46181;
    let t46195 = t7641 * t46185;
    let t46197 = t851 * t46116;
    let t46199 = t797 * t46121;
    (t46189, t46191, t46193, t46195, t46197, t46199)
}
