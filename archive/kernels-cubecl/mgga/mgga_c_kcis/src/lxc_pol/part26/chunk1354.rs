//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1354/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1354<F: Float>(t3717: F, t7086: F, t1385: F, t27370: F, t1364: F, t7052: F, t990: F, t29288: F, t94246: F, t7908: F, t103083: F, t27369: F, t28344: F, t28348: F, t28353: F, t28369: F, t28375: F, t28392: F, t7911: F, t98155: F, t98290: F) -> (F, F) {
    let t103155 = t3717 * t7086;
    let t103157 = t27370 * t103155 * t1385;
    let t103165 = t1364 * t7052 * t990;
    let t103172 = t94246 * t29288;
    let t103173 = t7908 * t103172;
    let t103185 = -F::cast_from(0.69505208333333333333e-3_f64) * t7908 * t103157 - F::cast_from(0.92754700520833333333e-4_f64) * t27369 * t103157 + F::cast_from(0.49555782539766601562e-5_f64) * t98290 * t103083 - F::cast_from(0.22653549382716049382e-2_f64) * t103165 * t7911 - F::cast_from(0.13901041666666666667e-2_f64) * t28369 * t28348 - F::cast_from(0.27802083333333333334e-2_f64) * t28369 * t28353 - F::cast_from(0.46336805555555555557e-3_f64) * t103173 + F::cast_from(0.74138888888888888888e-2_f64) * t28392 * t28375 + F::cast_from(0.37069444444444444444e-2_f64) * t28392 * t28344 + F::cast_from(0.49469173611111111111e-3_f64) * t98155 * t28344 + F::cast_from(0.37069444444444444444e-2_f64) * t28392 * t28348 + F::cast_from(0.74138888888888888888e-2_f64) * t28392 * t28353;
    (t103172, t103185)
}
