//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 900/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk900<F: Float>(t1818: F, t333: F, t3351: F, t511: F, t9210: F, t352: F, t515: F, t40294: F, t46873: F, t46875: F, t46877: F, t46879: F, t46881: F, t46883: F, t46885: F, t46889: F, t46892: F, t46894: F, t46899: F, t46904: F, t46906: F, t46911: F, t5055: F, t8374: F) -> (F,) {
    let t46916 = t3351 * t9210 * t511 * t1818 * t333;
    let t46921 = t3351 * t9210 * t515 * t1818 * t352;
    let t46925 = 0.85129199786595678796e-5 * t46873 - 0.85129199786595678796e-5 * t46875 - 0.85129199786595678796e-5 * t46877 - 0.85129199786595678796e-5 * t46879 + 0.25538759935978703638e-4 * t46881 - 0.76616279807936110914e-4 * t46883 + 0.10215503974391481455e-3 * t46885 - t40294 - 0.51077519871957407276e-4 * t46889 + 0.25538759935978703638e-4 * t46892 + 0.31923449919973379548e-4 * t46894 - 0.25538759935978703638e-4 * t46899 - 0.25538759935978703638e-4 * t46904 + 0.25538759935978703638e-4 * t46906 + 0.25538759935978703639e-4 * t46911 - 0.25538759935978703639e-4 * t46916 - 0.85129199786595678796e-5 * t46921 + 0.35922725105591425692e0 * t5055 * t8374;
    (t46925,)
}
