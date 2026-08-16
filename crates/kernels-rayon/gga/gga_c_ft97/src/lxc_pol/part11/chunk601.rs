//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 601/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk601(t7754: f64, t7771: f64, t7782: f64, t7786: f64, t7804: f64, t7820: f64, t8186: f64, t8192: f64, t8195: f64, t8260: f64, t8338: f64, t8348: f64, t8352: f64) -> f64 {
    let t8354 = 2.0_f64 / 9.0_f64 * t7782 - 2.0_f64 * t7786 + 4.0_f64 / 3.0_f64 * t7804 - 2.0_f64 / 3.0_f64 * t7820 - t8186 - 4.0_f64 / 3.0_f64 * t8192 + t8195 - 6.0_f64 * t7754 - 2.0_f64 * t7771 - t8260 + t8338 / 2.0_f64 + 3.0_f64 / 8.0_f64 * t8348 - 3.0_f64 / 4.0_f64 * t8352;
    t8354
}
