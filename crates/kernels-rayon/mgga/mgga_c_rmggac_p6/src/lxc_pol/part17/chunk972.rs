//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 972/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk972(t6434: f64, t649: f64, t7599: f64, t6394: f64, t36119: f64, t6397: f64, t41130: f64, t6400: f64, t8746: f64, t6382: f64, t36107: f64, t6387: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46092 = t649 * t6434;
    let t46093 = t7599 * t46092;
    let t46095 = t649 * t6394;
    let t46096 = t36119 * t46095;
    let t46098 = t649 * t6397;
    let t46099 = t41130 * t46098;
    let t46101 = t649 * t6400;
    let t46102 = t8746 * t46101;
    let t46106 = t649 * t6382;
    let t46107 = t36107 * t46106;
    let t46109 = t649 * t6387;
    (t46092, t46093, t46095, t46096, t46098, t46099, t46101, t46102, t46106, t46107, t46109)
}
