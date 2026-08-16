//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1097/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1097(t30490: f64, t30800: f64, t46873: f64, t46875: f64, t46877: f64, t46879: f64, t46881: f64, t46883: f64, t46885: f64, t46889: f64, t46892: f64, t46894: f64, t46899: f64, t46904: f64, t46906: f64, t46911: f64, t46916: f64, t46921: f64, t739: f64, t8041: f64, t884: f64) -> f64 {
    let t48818 = 0.1702583995731913576e-4_f64 * t46873 - 0.1702583995731913576e-4_f64 * t46875 - 0.1702583995731913576e-4_f64 * t46877 - 0.1702583995731913576e-4_f64 * t46879 + 0.5107751987195740728e-4_f64 * t46881 - 0.15323255961587222184e-3_f64 * t46883 + 0.20431007948782962912e-3_f64 * t46885 - 0.10215503974391481456e-3_f64 * t46889 + 0.5107751987195740728e-4_f64 * t46892 + 0.638468998399467591e-4_f64 * t46894 - 0.5107751987195740728e-4_f64 * t46899 - 0.5107751987195740728e-4_f64 * t46904 + 0.5107751987195740728e-4_f64 * t46906 + 0.5107751987195740728e-4_f64 * t46911 - 0.5107751987195740728e-4_f64 * t46916 - 0.1702583995731913576e-4_f64 * t46921 - 0.35922725105591425692e0_f64 * t739 * t8041 * t30800 + 0.35922725105591425692e0_f64 * t884 * t8041 * t30490;
    t48818
}
