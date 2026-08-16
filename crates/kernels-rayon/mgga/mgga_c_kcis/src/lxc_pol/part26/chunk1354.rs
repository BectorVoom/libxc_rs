//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1354/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1354(t3717: f64, t7086: f64, t1385: f64, t27370: f64, t1364: f64, t7052: f64, t990: f64, t29288: f64, t94246: f64, t7908: f64, t103083: f64, t27369: f64, t28344: f64, t28348: f64, t28353: f64, t28369: f64, t28375: f64, t28392: f64, t7911: f64, t98155: f64, t98290: f64) -> (f64, f64) {
    let t103155 = t3717 * t7086;
    let t103157 = t27370 * t103155 * t1385;
    let t103165 = t1364 * t7052 * t990;
    let t103172 = t94246 * t29288;
    let t103173 = t7908 * t103172;
    let t103185 = -0.69505208333333333333e-3_f64 * t7908 * t103157 - 0.92754700520833333333e-4_f64 * t27369 * t103157 + 0.49555782539766601562e-5_f64 * t98290 * t103083 - 0.22653549382716049382e-2_f64 * t103165 * t7911 - 0.13901041666666666667e-2_f64 * t28369 * t28348 - 0.27802083333333333334e-2_f64 * t28369 * t28353 - 0.46336805555555555557e-3_f64 * t103173 + 0.74138888888888888888e-2_f64 * t28392 * t28375 + 0.37069444444444444444e-2_f64 * t28392 * t28344 + 0.49469173611111111111e-3_f64 * t98155 * t28344 + 0.37069444444444444444e-2_f64 * t28392 * t28348 + 0.74138888888888888888e-2_f64 * t28392 * t28353;
    (t103172, t103185)
}
