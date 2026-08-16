//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 120/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk120(t11: f64, t1: f64, t351: f64, t21: f64, t84: f64, t352: f64, t354: f64, t30: f64, t347: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t356 = f64::sqrt(t11);
    let t357 = t356 * t1;
    let t358 = t357 * t351;
    let t360 = t21 * t84;
    let t362 = -0.632975e0_f64 * t352 - 0.29896666666666666667e0_f64 * t354 - 0.1023875e0_f64 * t358 - 0.82156666666666666667e-1_f64 * t360;
    let t363 = 1.0_f64 / t30;
    let t364 = t362 * t363;
    let t366 = 1.0_f64 * t347 * t364;
    (t357, t358, t360, t362, t363, t364, t366)
}
