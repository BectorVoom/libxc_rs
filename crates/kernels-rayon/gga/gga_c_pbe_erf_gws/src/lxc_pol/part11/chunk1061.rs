//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1061/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1061(t13428: f64, t20625: f64, t343: f64, t44741: f64, t2121: f64, t337: f64, t11563: f64, t3916: f64, t13300: f64, t6484: f64, t13349: f64, t6627: f64) -> (f64, f64, f64, f64, f64) {
    let t46151 = t20625 * t13428;
    let t46173 = t44741 * t343;
    let t46175 = t2121 * t337 * t46173;
    let t46199 = t3916 * t11563;
    let t46251 = t6484 * t13300;
    let t46253 = t6627 * t13349;
    (t46151, t46175, t46199, t46251, t46253)
}
