//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1258/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1258(t54329: f64, t14058: f64, t3279: f64, t1158: f64, t51395: f64, t3268: f64, t1140: f64, t14083: f64, t3190: f64, t3206: f64, t2407: f64, t26623: f64, t858: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54330 = 7.0_f64 / 24.0_f64 * t54329;
    let t54344 = t14058 * t3279;
    let t54345 = 35.0_f64 / 288.0_f64 * t54344;
    let t54352 = t51395 * t1158;
    let t54354 = t14058 * t3268;
    let t54355 = 7.0_f64 / 288.0_f64 * t54354;
    let t54356 = t14083 * t1140;
    let t54359 = t3206 * t3190;
    let t54373 = t2407 * t858 * t26623;
    (t54330, t54345, t54352, t54355, t54356, t54359, t54373)
}
