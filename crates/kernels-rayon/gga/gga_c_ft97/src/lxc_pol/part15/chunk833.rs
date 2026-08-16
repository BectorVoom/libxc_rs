//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 833/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk833(t1901: f64, t19449: f64, t19451: f64, t19453: f64, t19482: f64, t19484: f64, t19504: f64, t19511: f64, t22218: f64, t22222: f64, t22226: f64, t22230: f64, t22242: f64, t22246: f64, t22251: f64, t22255: f64, t446: f64) -> f64 {
    let t22258 = -t446 * t22218 / 9.0_f64 - 10.0_f64 / 81.0_f64 * t446 * t22222 - t446 * t22226 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t446 * t22230 + 2.0_f64 / 3.0_f64 * t19449 + t19451 / 3.0_f64 + t19453 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t19482 - 2.0_f64 / 3.0_f64 * t19484 - 2.0_f64 / 9.0_f64 * t19504 - 2.0_f64 / 9.0_f64 * t19511 - 2.0_f64 / 3.0_f64 * t1901 * t22242 - 2.0_f64 / 3.0_f64 * t1901 * t22246 - 2.0_f64 * t446 * t22251 + 4.0_f64 / 9.0_f64 * t446 * t22255;
    t22258
}
