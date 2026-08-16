//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1121/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1121(t41448: f64, t41911: f64, t43480: f64, t89: f64, t39370: f64, t666: f64, t792: f64, t10415: f64, t9725: f64, t10271: f64, t41962: f64, t295: f64, t41446: f64) -> (f64, f64, f64, f64, f64) {
    let t43483 = t89 * t41911 * t43480 * t41448;
    let t43487 = t89 * t666 * t792 * t39370;
    let t43490 = t89 * t9725 * t10415;
    let t43493 = t89 * t41962 * t10271;
    let t43495 = t295 * t41446;
    (t43483, t43487, t43490, t43493, t43495)
}
