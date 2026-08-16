//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1022/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1022(t39: f64, t55: f64, t59: f64, t87: f64, t1759: f64, t1784: f64, t1790: f64, t1792: f64, t533: f64, t6446: f64, t1758: f64, t6452: f64, t6454: f64) -> (f64, f64, f64, f64) {
    let t22274 = 24.0_f64 * t39 * t55 * t59 * t87;
    let t22277 = 36.0_f64 * t1790 * t1759 * t1784;
    let t22281 = 0.64327297288604419288e2_f64 * t1790 * t6446 * t1792 * t533;
    let t22285 = 0.3103500882342370105e4_f64 * t6452 * t1758 * t6454 * t1784;
    (t22274, t22277, t22281, t22285)
}
