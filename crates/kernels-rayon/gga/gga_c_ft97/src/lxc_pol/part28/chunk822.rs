//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 822/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk822(t33105: f64, t33118: f64, t143: f64, t160: f64, t1359: f64, t5968: f64, t574: f64, t605: f64, t1901: f64, t28: f64, t33052: f64, t33057: f64, t33062: f64, t33066: f64, t33068: f64, t33072: f64, t33077: f64, t33082: f64, t33087: f64, t33092: f64, t446: f64, t89: f64) -> (f64, f64, f64, f64, f64) {
    let t33119 = t33105 + t33118;
    let t33121 = t143 * t33119 * t160;
    let t33125 = t1359 * t5968;
    let t33127 = t574 * t605 * t33125;
    let t33130 = t446 * t33052 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t33057 + 2.0_f64 / 3.0_f64 * t446 * t33062 - t33066 - 2.0_f64 / 9.0_f64 * t1901 * t33068 + 2.0_f64 / 3.0_f64 * t446 * t33072 + t446 * t33077 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t33082 - 2.0_f64 * t446 * t33087 + 4.0_f64 / 3.0_f64 * t446 * t33092 + t89 * t28 * t33121 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t33127;
    (t33119, t33121, t33125, t33127, t33130)
}
