//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1296/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1296(t39411: f64, t39413: f64, t39418: f64, t49240: f64, t49242: f64, t49271: f64, t49273: f64, t56966: f64, t56969: f64, t56972: f64, t56975: f64, t56978: f64, t56981: f64, t56984: f64) -> f64 {
    let t57135 = -0.80513333333333333336e0_f64 * t39411 - 0.53675555555555555556e0_f64 * t39413 + 0.16102666666666666667e1_f64 * t39418 + 0.80513333333333333333e0_f64 * t49240 - 0.24154e1_f64 * t49242 - 0.132456e1_f64 * t49271 + 0.22076e0_f64 * t49273 + 0.72462e1_f64 * t56966 - 0.20128333333333333334e1_f64 * t56969 - 0.11038e0_f64 * t56972 - 0.22076e0_f64 * t56975 - 0.108693e2_f64 * t56978 + 0.24154e1_f64 * t56981 - 0.80513333333333333332e0_f64 * t56984;
    t57135
}
