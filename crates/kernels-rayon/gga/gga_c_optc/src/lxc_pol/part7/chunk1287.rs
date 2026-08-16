//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1287/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1287(t26252: f64, t26258: f64, t26278: f64, t26280: f64, t26284: f64, t26289: f64, t26293: f64, t26296: f64, t26300: f64, t26304: f64, t26306: f64, t522: f64, t8656: f64) -> (f64, f64) {
    let t26308 = 40.0_f64 / 81.0_f64 * t26252 + 40.0_f64 / 9.0_f64 * t26258 - 20.0_f64 / 9.0_f64 * t26278 + 8.0_f64 / 3.0_f64 * t26280 - 8.0_f64 * t26284 + 8.0_f64 * t26289 - 2.0_f64 / 3.0_f64 * t26293 + 8.0_f64 * t26296 - 12.0_f64 * t26300 + 2.0_f64 * t26304 - 8.0_f64 / 3.0_f64 * t26306;
    let t26309 = t522 * t8656;
    (t26308, t26309)
}
