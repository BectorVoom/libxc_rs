//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 652/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk652(t108: f64, t3342: f64, t3346: f64, t3351: f64, t3354: f64, t726: f64, t728: f64, t92: f64, t93: f64, t1902: f64, t1905: f64, t1920: f64, t1926: f64, t267: f64, t3498: f64, t3502: f64, t3506: f64, t3507: f64, t3508: f64, t3509: f64, t3510: f64) -> (f64, f64) {
    let t3603 = (20.0_f64 / 9.0_f64 * t92 * t3342 + 4.0_f64 / 3.0_f64 * t726 * t3346 + 20.0_f64 / 9.0_f64 * t93 * t3351 + 4.0_f64 / 3.0_f64 * t728 * t3354) * t108;
    let t3606 = t3498 - t3502 - t3506 - t3507 + t3508 - t3509 - t3603 * t267 / 15.0_f64 + t3510 + t1902 - t1905 + t1920 + t1926;
    (t3603, t3606)
}
