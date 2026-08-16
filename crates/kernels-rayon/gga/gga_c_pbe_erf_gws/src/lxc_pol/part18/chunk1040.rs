//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1040/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1040(t11651: f64, t9665: f64, t3257: f64, t3803: f64, t6355: f64, t326: f64, t6469: f64, t820: f64, t339: f64, t3802: f64, t6472: f64, t860: f64) -> (f64, f64, f64, f64, f64) {
    let t11652 = t9665 * t11651;
    let t11656 = t3257 * t3803 * t6355;
    let t11660 = t326 * t6469 * t820;
    let t11661 = t3802 * t339;
    let t11662 = t6472 * t11661;
    let t11663 = t11660 * t11662;
    let t11665 = t11663 * t860 / 96.0_f64;
    (t11652, t11656, t11660, t11661, t11665)
}
