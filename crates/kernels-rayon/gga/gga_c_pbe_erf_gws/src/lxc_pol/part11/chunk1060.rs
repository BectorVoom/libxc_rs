//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1060/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1060(t13379: f64, t6627: f64, t13433: f64, t9630: f64, t13599: f64, t6501: f64, t11592: f64, t11868: f64, t13496: f64, t6484: f64, t13371: f64, t6542: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46013 = t6627 * t13379;
    let t46023 = t9630 * t13433;
    let t46078 = t6501 * t13599;
    let t46098 = t11592 * t11868;
    let t46104 = t6484 * t13496;
    let t46115 = t6542 * t13371;
    (t46013, t46023, t46078, t46098, t46104, t46115)
}
