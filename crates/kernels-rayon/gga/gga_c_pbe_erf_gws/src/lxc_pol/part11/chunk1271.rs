//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1271/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1271(t46536: f64, t48985: f64, t858: f64, t884: f64, t886: f64, t11414: f64, t37965: f64, t13252: f64, t39052: f64, t46549: f64, t46566: f64, t11540: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t50275 = 7.0_f64 / 72.0_f64 * t46536;
    let t50279 = t884 * t886 * t858 * t48985 / 48.0_f64;
    let t50281 = t37965 * t11414 / 4.0_f64;
    let t50286 = t39052 * t13252;
    let t50290 = 7.0_f64 / 72.0_f64 * t46549;
    let t50291 = 7.0_f64 / 24.0_f64 * t46566;
    let t50292 = t11540 * t13252;
    (t50275, t50279, t50281, t50286, t50290, t50291, t50292)
}
