//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 915/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk915(t37427: f64, t37433: f64, t38257: f64, t38260: f64, t38266: f64, t38271: f64, t38275: f64, t38279: f64, t38281: f64, t38285: f64, t38288: f64, t38292: f64, t38449: f64, t38459: f64, t38635: f64) -> f64 {
    let t38825 = 8.0_f64 / 3.0_f64 * t37427 + 8.0_f64 * t37433 - t38257 / 3.0_f64 - 8.0_f64 / 9.0_f64 * t38260 + 8.0_f64 / 3.0_f64 * t38266 - 8.0_f64 / 9.0_f64 * t38271 - 16.0_f64 / 9.0_f64 * t38275 - 4.0_f64 / 3.0_f64 * t38279 - 4.0_f64 / 9.0_f64 * t38281 - 4.0_f64 / 3.0_f64 * t38285 + 4.0_f64 / 27.0_f64 * t38288 - 8.0_f64 / 3.0_f64 * t38292 - t38449 / 4.0_f64 - 5.0_f64 / 16.0_f64 * t38459 + t38635 / 6.0_f64;
    t38825
}
