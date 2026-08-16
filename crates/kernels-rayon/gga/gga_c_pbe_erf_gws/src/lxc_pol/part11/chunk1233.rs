//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1233/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1233(t1076: f64, t3824: f64, t11630: f64, t11794: f64, t11592: f64, t13243: f64, t36869: f64, t1134: f64, t3772: f64, t44889: f64, t12041: f64, t46544: f64, t860: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49491 = t1076 * t3824;
    let t49498 = t11794 * t11630 / 16.0_f64;
    let t49500 = t11592 * t13243 / 6.0_f64;
    let t49507 = 35.0_f64 / 36.0_f64 * t36869;
    let t49508 = t1134 * t3772;
    let t49514 = 7.0_f64 / 4.0_f64 * t44889;
    let t49521 = t12041 * t46544 * t860 / 24.0_f64;
    (t49491, t49498, t49500, t49507, t49508, t49514, t49521)
}
