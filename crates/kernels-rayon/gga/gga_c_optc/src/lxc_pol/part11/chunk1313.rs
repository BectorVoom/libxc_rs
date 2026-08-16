//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1313/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1313(t39545: f64, t39560: f64, t49395: f64, t57012: f64, t57016: f64, t57020: f64, t57024: f64, t57027: f64, t57030: f64, t57034: f64, t57037: f64, t57041: f64, t57044: f64, t57048: f64) -> f64 {
    let t57432 = 0.76514814814814814814e0_f64 * t49395 + 0.250068e1_f64 * t57012 + 0.62517e0_f64 * t57016 - 0.104195e0_f64 * t57020 - 0.123954e2_f64 * t57024 - 0.103295e1_f64 * t57027 - 0.125034e1_f64 * t57030 + 0.55570666666666666666e0_f64 * t57034 + 0.68863333333333333334e1_f64 * t57037 - 0.15302962962962962963e1_f64 * t57041 - 0.10805407407407407407e0_f64 * t57044 - 0.104195e0_f64 * t57048 - 0.23154444444444444445e0_f64 * t39545 - 0.69463333333333333334e0_f64 * t39560;
    t57432
}
