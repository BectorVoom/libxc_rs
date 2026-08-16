//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 883/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk883(t255: f64, t42123: f64, t41950: f64, t761: f64, t9577: f64, t259: f64, t41743: f64, t89: f64, t327: f64, t41446: f64, t170: f64, t328: f64, t39600: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42517 = t42123 * t255;
    let t42759 = 280.0_f64 / 81.0_f64 * t41950;
    let t42859 = t761 * t9577;
    let t42928 = 280.0_f64 / 243.0_f64 * t89 * t41743 * t259;
    let t43050 = t327 * t41446;
    let t43084 = 220.0_f64 / 81.0_f64 * t170 * t39600 * t328;
    (t42517, t42759, t42859, t42928, t43050, t43084)
}
