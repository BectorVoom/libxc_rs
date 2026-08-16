//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1298/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1298(t26309: f64, t26311: f64, t26313: f64, t26314: f64, t26319: f64, t26324: f64, t26326: f64, t26328: f64, t26330: f64, t26332: f64, t26339: f64, t26343: f64) -> f64 {
    let t26345 = 8.0_f64 / 9.0_f64 * t26309 - 16.0_f64 / 9.0_f64 * t26311 + t26313 + 4.0_f64 / 9.0_f64 * t26314 + 8.0_f64 / 3.0_f64 * t26319 - 8.0_f64 / 9.0_f64 * t26324 - 8.0_f64 / 9.0_f64 * t26326 - 16.0_f64 / 27.0_f64 * t26328 + 16.0_f64 / 9.0_f64 * t26330 + 112.0_f64 / 81.0_f64 * t26332 - 80.0_f64 / 81.0_f64 * t26339 - t26343 / 3.0_f64;
    t26345
}
