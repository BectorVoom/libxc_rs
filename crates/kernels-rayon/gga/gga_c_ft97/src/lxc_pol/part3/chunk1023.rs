//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1023/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1023(t14895: f64, t14951: f64, t19004: f64, t19008: f64, t19013: f64, t19018: f64, t19022: f64, t19025: f64, t19028: f64, t19032: f64, t19243: f64, t19246: f64) -> (f64, f64) {
    let t19836 = -4.0_f64 / 3.0_f64 * t19004 + 4.0_f64 / 9.0_f64 * t19008 - 8.0_f64 / 9.0_f64 * t14895 + t14951 + t19013 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t19018 - 2.0_f64 / 3.0_f64 * t19022 - 2.0_f64 * t19025 - 8.0_f64 / 3.0_f64 * t19028 + t19032 / 3.0_f64 - t19243;
    let t19838 = t19246 / 3.0_f64;
    (t19836, t19838)
}
