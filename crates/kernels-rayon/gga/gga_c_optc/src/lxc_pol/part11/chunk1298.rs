//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1298/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1298(t39545: f64, t39560: f64, t49395: f64, t57012: f64, t57016: f64, t57020: f64, t57024: f64, t57027: f64, t57030: f64, t57034: f64, t57037: f64, t57041: f64, t57044: f64, t57048: f64) -> f64 {
    let t57164 = 0.44729629629629629629e0_f64 * t49395 + 0.198684e1_f64 * t57012 + 0.49671e0_f64 * t57016 - 0.82785e-1_f64 * t57020 - 0.72462e1_f64 * t57024 - 0.60384999999999999999e0_f64 * t57027 - 0.99342e0_f64 * t57030 + 0.44152e0_f64 * t57034 + 0.40256666666666666666e1_f64 * t57037 - 0.89459259259259259259e0_f64 * t57041 - 0.8585111111111111111e-1_f64 * t57044 - 0.82785e-1_f64 * t57048 - 0.18396666666666666667e0_f64 * t39545 - 0.5519e0_f64 * t39560;
    t57164
}
