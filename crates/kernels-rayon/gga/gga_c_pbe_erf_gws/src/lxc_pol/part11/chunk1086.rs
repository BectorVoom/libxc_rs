//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1086/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1086(t17678: f64, t17758: f64, t25: f64, t31643: f64, t47392: f64, t47396: f64, t47401: f64, t47407: f64, t47412: f64, t47416: f64, t47420: f64, t47423: f64, t47426: f64, t5264: f64, t606: f64) -> f64 {
    let t47428 = 0.19195555555555555555e0_f64 * t31643 + 0.35555555555555555554e-1_f64 * t25 * t5264 * t47392 - 0.69135802469135802468e-2_f64 * t25 * t17758 * t47396 - 0.66666666666666666667e-2_f64 * t25 * t606 * t47401 + t17678 - 0.86380000000000000002e0_f64 * t47407 - 0.71983333333333333335e-1_f64 * t47412 + 0.8638e0_f64 * t47416 + 0.21595e0_f64 * t47420 + 0.4798888888888888889e0_f64 * t47423 - 0.10664197530864197531e0_f64 * t47426;
    t47428
}
