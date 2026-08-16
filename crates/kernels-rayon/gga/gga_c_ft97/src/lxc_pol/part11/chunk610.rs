//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 610/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk610(t7782: f64, t7820: f64, t8192: f64, t8195: f64, t7771: f64, t8189: f64, t7754: f64, t7786: f64, t7804: f64, t8186: f64, t8338: f64, t8348: f64, t8352: f64) -> f64 {
    let t8446 = 2.0_f64 / 27.0_f64 * t7782;
    let t8449 = 2.0_f64 / 9.0_f64 * t7820;
    let t8451 = 4.0_f64 / 9.0_f64 * t8192;
    let t8452 = t8195 / 3.0_f64;
    let t8454 = 2.0_f64 / 3.0_f64 * t7771;
    let t8455 = 28.0_f64 / 81.0_f64 * t8189;
    let t8459 = t8446 - 2.0_f64 / 3.0_f64 * t7786 + 4.0_f64 / 9.0_f64 * t7804 - t8449 - t8186 / 3.0_f64 - t8451 + t8452 - 2.0_f64 * t7754 - t8454 - t8455 + t8338 / 6.0_f64 + t8348 / 8.0_f64 - t8352 / 4.0_f64;
    t8459
}
