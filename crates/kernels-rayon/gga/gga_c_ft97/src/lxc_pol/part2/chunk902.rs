//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 902/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk902(t13722: f64, t13700: f64, t13704: f64, t13708: f64, t13719: f64, t9701: f64, t9735: f64, t9861: f64, t9862: f64, t9869: f64, t9870: f64, t13739: f64) -> (f64, f64) {
    let t13976 = 4.0_f64 / 27.0_f64 * t13722;
    let t13977 = t13700 / 2.0_f64 - 4.0_f64 / 3.0_f64 * t13704 + 4.0_f64 / 9.0_f64 * t13708 + t9861 + t9862 - 8.0_f64 / 27.0_f64 * t9735 - 8.0_f64 / 9.0_f64 * t9701 - t9869 + t9870 - 6.0_f64 * t13719 - t13976;
    let t13981 = 4.0_f64 / 9.0_f64 * t13739;
    (t13977, t13981)
}
