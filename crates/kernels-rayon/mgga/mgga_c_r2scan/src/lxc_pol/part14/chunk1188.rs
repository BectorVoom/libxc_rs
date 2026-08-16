//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1188/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1188(t10940: f64, t12033: f64, t10634: f64, t12098: f64, t3262: f64, t3465: f64, t40383: f64, t11336: f64, t37327: f64, t40297: f64, t3719: f64, t792: f64) -> (f64, f64, f64, f64, f64) {
    let t41179 = t10940 * t12033 / 4.0_f64;
    let t41182 = 15.0_f64 / 8.0_f64 * t3262 * t12098 * t10634;
    let t41185 = 3.0_f64 / 2.0_f64 * t3262 * t3465 * t40383;
    let t41188 = 15.0_f64 / 8.0_f64 * t37327 * t11336 * t40297;
    let t41189 = t3719 * t792;
    (t41179, t41182, t41185, t41188, t41189)
}
