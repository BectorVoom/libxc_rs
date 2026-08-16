//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1100/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1100(t13792: f64, t13984: f64, t2201: f64, t326: f64, t378: f64, t2409: f64, t4016: f64, t8734: f64, t4052: f64, t938: f64, t3067: f64, t4009: f64, t4414: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13985 = t13792 * t13984;
    let t13987 = t326 * t2201;
    let t13988 = t13987 * t378;
    let t13989 = 35.0_f64 / 432.0_f64 * t13988;
    let t13991 = t2409 * t8734 * t4016;
    let t13994 = t4052 * t938;
    let t13996 = t2409 * t3067 * t13994;
    let t13999 = t4414 * t4009;
    (t13985, t13987, t13989, t13991, t13994, t13996, t13999)
}
