//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 615/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk615(t100: f64, t8326: f64, t3194: f64, t8205: f64, t1822: f64, t1882: f64, t1901: f64, t446: f64, t8475: f64, t8477: f64, t8480: f64, t8483: f64, t8485: f64, t8487: f64, t8491: f64, t8496: f64, t8499: f64, t8503: f64, t8507: f64, t8512: f64, t8516: f64) -> (f64, f64, f64, f64) {
    let t8518 = t8326 * t100;
    let t8519 = t3194 * t8205;
    let t8520 = t8518 * t8519;
    let t8523 = t1882 * t1822;
    let t8525 = -4.0_f64 / 9.0_f64 * t8475 + t8477 / 3.0_f64 + 2.0_f64 * t446 * t8480 + 2.0_f64 / 3.0_f64 * t8483 - 4.0_f64 / 9.0_f64 * t8485 - 2.0_f64 / 3.0_f64 * t8487 + t1901 * t8491 / 3.0_f64 + t1901 * t8496 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t8499 + 2.0_f64 * t446 * t8503 + 2.0_f64 / 3.0_f64 * t1901 * t8507 + t1901 * t8512 / 3.0_f64 + 4.0_f64 / 9.0_f64 * t8516 + 2.0_f64 / 9.0_f64 * t1901 * t8520 + t8523 / 3.0_f64;
    (t8518, t8519, t8520, t8525)
}
