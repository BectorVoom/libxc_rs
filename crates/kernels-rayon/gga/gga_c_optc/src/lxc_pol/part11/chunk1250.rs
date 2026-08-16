//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1250/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1250(t22716: f64, t22719: f64, t22724: f64, t23431: f64, t23438: f64, t39066: f64, t4595: f64, t48009: f64, t56295: f64, t56296: f64, t56297: f64, t56299: f64, t95: f64) -> f64 {
    let t56667 = -t22716 - t22719 + t56295 + t56296 + t23431 - t23438 + 70.0_f64 / 3.0_f64 * t39066 - t56297 + t22724 + 0.93041573165652349788e-1_f64 * t95 * t48009 * t4595 + t56299;
    t56667
}
