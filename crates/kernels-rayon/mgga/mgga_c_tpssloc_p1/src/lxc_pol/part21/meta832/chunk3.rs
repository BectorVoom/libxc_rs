//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2935/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2935(t48155: f64, t48157: f64, t48159: f64, t48161: f64, t48163: f64, t48165: f64, t48167: f64, t60163: f64, t60166: f64, t60168: f64, t60171: f64, t60173: f64, t60189: f64) -> f64 {
    let t61138 = -2.0_f64 / 9.0_f64 * t60163 + t60166 / 6.0_f64 - 10.0_f64 / 27.0_f64 * t60168 - 2.0_f64 / 3.0_f64 * t60171 + 5.0_f64 / 27.0_f64 * t60173 - 40.0_f64 / 27.0_f64 * t48155 + 20.0_f64 / 81.0_f64 * t48157 + 8.0_f64 / 9.0_f64 * t48159 + 4.0_f64 / 9.0_f64 * t48161 + 4.0_f64 / 9.0_f64 * t48163 - 4.0_f64 / 27.0_f64 * t48165 - 2.0_f64 / 27.0_f64 * t48167 - 4.0_f64 * t60189;
    t61138
}
