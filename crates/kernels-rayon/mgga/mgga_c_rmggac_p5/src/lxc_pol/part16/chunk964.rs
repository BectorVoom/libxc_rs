//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 964/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk964(t36107: f64, t46083: f64, t6412: f64, t649: f64, t8764: f64, t6449: f64, t7599: f64, t6434: f64, t6394: f64, t36119: f64, t6397: f64, t41130: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46084 = t36107 * t46083;
    let t46086 = t649 * t6412;
    let t46087 = t8764 * t46086;
    let t46089 = t649 * t6449;
    let t46090 = t7599 * t46089;
    let t46092 = t649 * t6434;
    let t46093 = t7599 * t46092;
    let t46095 = t649 * t6394;
    let t46096 = t36119 * t46095;
    let t46098 = t649 * t6397;
    let t46099 = t41130 * t46098;
    (t46084, t46086, t46087, t46089, t46090, t46092, t46093, t46095, t46096, t46098, t46099)
}
