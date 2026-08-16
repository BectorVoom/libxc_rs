//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 917/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk917(t11587: f64, t1647: f64, t1901: f64, t1909: f64, t1922: f64, t379: f64, t38846: f64, t38866: f64, t446: f64, t447: f64, t487: f64, t8206: f64, t8212: f64, t8355: f64, t8372: f64, t8387: f64, t8425: f64, t8506: f64, t8510: f64, t8511: f64, t8519: f64, t8535: f64) -> f64 {
    let t38883 = -112.0_f64 / 81.0_f64 * t38846 - 4.0_f64 / 3.0_f64 * t1901 * t1909 * t8510 * t1647 + 4.0_f64 / 9.0_f64 * t1901 * t1909 * t487 * t8355 * t379 + 4.0_f64 / 3.0_f64 * t1901 * t8506 * t8511 + 8.0_f64 / 9.0_f64 * t1901 * t11587 * t8212 + 4.0_f64 / 3.0_f64 * t1901 * t8372 * t8387 + 8.0_f64 / 9.0_f64 * t1901 * t38866 * t8519 - 8.0_f64 / 3.0_f64 * t1901 * t8506 * t8425 + 8.0_f64 / 3.0_f64 * t1901 * t8506 * t8535 - 8.0_f64 / 3.0_f64 * t1901 * t8372 * t8206 + 4.0_f64 / 3.0_f64 * t446 * t447 * t1922 * t1647;
    t38883
}
