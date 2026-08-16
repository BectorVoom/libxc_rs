//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 914/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk914(t37365: f64, t37368: f64, t37372: f64, t37376: f64, t37379: f64, t37382: f64, t37385: f64, t37394: f64, t37399: f64, t37403: f64, t37410: f64, t37413: f64, t37418: f64, t37421: f64, t37424: f64) -> f64 {
    let t38809 = -2.0_f64 / 9.0_f64 * t37365 + 8.0_f64 / 9.0_f64 * t37368 + 8.0_f64 / 3.0_f64 * t37372 + 2.0_f64 / 3.0_f64 * t37376 - 16.0_f64 / 27.0_f64 * t37379 + 112.0_f64 / 243.0_f64 * t37382 + 16.0_f64 / 27.0_f64 * t37385 - t37394 / 9.0_f64 - 12.0_f64 * t37399 + 40.0_f64 / 243.0_f64 * t37403 + 40.0_f64 / 27.0_f64 * t37410 + 112.0_f64 / 81.0_f64 * t37413 + 2.0_f64 * t37418 + 16.0_f64 / 9.0_f64 * t37421 + 4.0_f64 / 9.0_f64 * t37424;
    t38809
}
