//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1293/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1293(t23860: f64, t30270: f64, t39413: f64, t39418: f64, t49240: f64, t49242: f64, t49393: f64, t49395: f64, t56969: f64, t57027: f64, t57037: f64, t57041: f64) -> f64 {
    let t57098 = -2.0_f64 / 3.0_f64 * t57027 - 20.0_f64 / 9.0_f64 * t56969 - 16.0_f64 / 27.0_f64 * t39413 + 16.0_f64 / 9.0_f64 * t39418 + t23860 + 8.0_f64 / 9.0_f64 * t49240 - 8.0_f64 / 3.0_f64 * t49242 + 4.0_f64 / 9.0_f64 * t49393 + 40.0_f64 / 81.0_f64 * t49395 + 112.0_f64 / 81.0_f64 * t30270 + 40.0_f64 / 9.0_f64 * t57037 - 80.0_f64 / 81.0_f64 * t57041;
    t57098
}
