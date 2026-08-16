//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1146/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1146(t11032: f64, t3407: f64, t12631: f64, t7130: f64, t12801: f64, t12748: f64, t7527: f64, t41702: f64, t41769: f64, t41772: f64, t48213: f64, t48215: f64, t48219: f64, t48223: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48225 = 16.0_f64 / 15.0_f64 * t11032 * t3407;
    let t48227 = 32.0_f64 / 15.0_f64 * t7130 * t12631;
    let t48229 = 32.0_f64 / 9.0_f64 * t7130 * t12801;
    let t48231 = 64.0_f64 / 15.0_f64 * t7527 * t12748;
    let t48232 = 64.0_f64 / 27.0_f64 * t41702;
    let t48233 = 32.0_f64 / 27.0_f64 * t41769;
    let t48234 = 64.0_f64 / 27.0_f64 * t41772;
    let t48235 = -t48213 + t48215 + t48219 + t48223 - t48225 + t48227 + t48229 - t48231 + t48232 + t48233 + t48234;
    (t48225, t48227, t48229, t48231, t48232, t48233, t48234, t48235)
}
