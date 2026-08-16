//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 642/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk642(t184: f64, t3492: f64, t221: f64, t2747: f64, t1010: f64, t2615: f64, t1891: f64, t3351: f64, t642: f64, t639: f64, t1896: f64, t3342: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3493 = t3492 * t184;
    let t3495 = 4.0_f64 / 15.0_f64 * t3493 * t221;
    let t3496 = 8.0_f64 / 45.0_f64 * t2747;
    let t3498 = 8.0_f64 / 45.0_f64 * t2615 * t1010;
    let t3499 = t1891 * t3351;
    let t3500 = t642 * t3499;
    let t3502 = 8.0_f64 / 45.0_f64 * t639 * t3500;
    let t3503 = t1896 * t3342;
    (t3493, t3495, t3496, t3498, t3499, t3500, t3502, t3503)
}
