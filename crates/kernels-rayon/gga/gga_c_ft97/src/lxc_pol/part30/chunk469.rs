//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 469/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk469(t7553: f64, t762: f64, t242: f64, t193: f64, t446: f64, t7495: f64, t7499: f64, t7504: f64, t7508: f64, t7538: f64, t7543: f64, t7548: f64, t89: f64) -> (f64, f64, f64) {
    let t7554 = t762 * t7553;
    let t7555 = t242 * t7554;
    let t7558 = 2.0_f64 / 3.0_f64 * t446 * t7495 - 2.0_f64 / 3.0_f64 * t446 * t7499 + 2.0_f64 / 3.0_f64 * t446 * t7504 - t446 * t7508 / 3.0_f64 + t89 * t193 * t7538 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t7543 + 2.0_f64 / 3.0_f64 * t446 * t7548 - t446 * t7555 / 3.0_f64;
    (t7554, t7555, t7558)
}
