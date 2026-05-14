//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 961/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk961<F: Float>(t1663: F, t47409: F, t11: F, t1758: F, t47391: F, t4957: F, t571: F, t1764: F, t47392: F, t4949: F, t16738: F, t47396: F, t17678: F, t17758: F, t25: F, t31643: F, t47401: F, t47407: F, t5264: F, t606: F) -> (F, F, F, F, F, F, F, F, F) {
    let t47410 = t1663 * t47409;
    let t47412 = t11 * t1758 * t47410;
    let t47414 = t4957 * t47391;
    let t47416 = t11 * t571 * t47414;
    let t47418 = t1764 * t47409;
    let t47420 = t11 * t571 * t47418;
    let t47423 = t11 * t4949 * t47392;
    let t47426 = t11 * t16738 * t47396;
    let t47428 = 0.19195555555555555555e0 * t31643 + 0.35555555555555555554e-1 * t25 * t5264 * t47392 - 0.69135802469135802468e-2 * t25 * t17758 * t47396 - 0.66666666666666666667e-2 * t25 * t606 * t47401 + t17678 - 0.86380000000000000002e0 * t47407 - 0.71983333333333333335e-1 * t47412 + 0.8638e0 * t47416 + 0.21595e0 * t47420 + 0.4798888888888888889e0 * t47423 - 0.10664197530864197531e0 * t47426;
    (t47410, t47412, t47414, t47416, t47418, t47420, t47423, t47426, t47428)
}
