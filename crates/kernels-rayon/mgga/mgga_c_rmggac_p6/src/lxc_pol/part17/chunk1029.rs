//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1029/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1029(t40294: f64, t46873: f64, t46875: f64, t46877: f64, t46879: f64, t46881: f64, t46883: f64, t46885: f64, t46889: f64, t46892: f64, t46894: f64, t46899: f64, t46904: f64, t46906: f64, t46911: f64, t46916: f64, t46921: f64, t5055: f64, t8374: f64) -> f64 {
    let t46925 = 0.85129199786595678796e-5_f64 * t46873 - 0.85129199786595678796e-5_f64 * t46875 - 0.85129199786595678796e-5_f64 * t46877 - 0.85129199786595678796e-5_f64 * t46879 + 0.25538759935978703638e-4_f64 * t46881 - 0.76616279807936110914e-4_f64 * t46883 + 0.10215503974391481455e-3_f64 * t46885 - t40294 - 0.51077519871957407276e-4_f64 * t46889 + 0.25538759935978703638e-4_f64 * t46892 + 0.31923449919973379548e-4_f64 * t46894 - 0.25538759935978703638e-4_f64 * t46899 - 0.25538759935978703638e-4_f64 * t46904 + 0.25538759935978703638e-4_f64 * t46906 + 0.25538759935978703639e-4_f64 * t46911 - 0.25538759935978703639e-4_f64 * t46916 - 0.85129199786595678796e-5_f64 * t46921 + 0.35922725105591425692e0_f64 * t5055 * t8374;
    t46925
}
