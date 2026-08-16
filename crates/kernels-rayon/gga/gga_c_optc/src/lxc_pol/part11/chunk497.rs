//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 497/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk497(t3313: f64, t3315: f64, t115: f64, t56: f64, t5: f64, t1261: f64, t2007: f64, t1235: f64, t1933: f64, t1239: f64, t1940: f64, t1271: f64, t2024: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3316 = t3313 * t3315;
    let t3317 = t56 * t115;
    let t3318 = t3317 * t5;
    let t3325 = t2007 * t1261;
    let t3331 = t1933 * t1235;
    let t3339 = t1940 * t1239;
    let t3353 = t1271 * t2024;
    (t3316, t3318, t3325, t3331, t3339, t3353)
}
