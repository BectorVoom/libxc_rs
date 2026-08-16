//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 596/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk596(t1806: f64, t458: f64, t462: f64, t8263: f64, t8267: f64, t8272: f64, t8278: f64, t8283: f64, t8285: f64, t8287: f64, t8289: f64, t8292: f64, t8295: f64, t92: f64) -> f64 {
    let t8298 = t458 * t1806;
    let t8299 = 6.0_f64 * t462 * t8263 - t462 * t8267 / 3.0_f64 - 6.0_f64 * t92 * t8272 - 10.0_f64 / 27.0_f64 * t462 * t8278 - 4.0_f64 / 9.0_f64 * t8283 + t8285 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t8287 - 2.0_f64 * t8289 - 2.0_f64 * t462 * t8292 - 2.0_f64 * t462 * t8295 + t8298;
    t8299
}
