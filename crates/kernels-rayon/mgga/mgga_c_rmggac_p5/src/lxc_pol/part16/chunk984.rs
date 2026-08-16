//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 984/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk984(t41404: f64, t46106: f64, t40999: f64, t46109: f64, t35960: f64, t649: f64, t6530: f64, t41407: f64, t6561: f64, t6564: f64, t40928: f64, t6523: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46471 = t41404 * t46106;
    let t46473 = t40999 * t46109;
    let t46476 = t35960 * t649 * t6530;
    let t46480 = t41407 * t649 * t6561;
    let t46483 = t35960 * t649 * t6564;
    let t46486 = t40928 * t649 * t6523;
    (t46471, t46473, t46476, t46480, t46483, t46486)
}
