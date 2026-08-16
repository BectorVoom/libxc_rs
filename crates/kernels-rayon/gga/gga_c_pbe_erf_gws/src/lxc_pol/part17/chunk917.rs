//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 917/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk917(t4810: f64, t4817: f64, t2513: f64, t409: f64, t2515: f64, t414: f64, t1336: f64, t960: f64, t1396: f64, t2840: f64, t1392: f64, t1: f64, t2474: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8008 = 0.48830813431341759842e-3_f64 * t4810;
    let t8009 = 0.18311555036753159941e-3_f64 * t4817;
    let t8010 = t409 * t2513;
    let t8011 = 8.0_f64 * t8010;
    let t8012 = t414 * t2515;
    let t8013 = 8.0_f64 * t8012;
    let t8014 = t1336 * t960;
    let t8015 = 12.0_f64 * t8014;
    let t8016 = t2840 * t1396;
    let t8017 = 0.58482233974552040708e0_f64 * t8016;
    let t8018 = t2840 * t1392;
    let t8019 = 0.17315755899375863299e2_f64 * t8018;
    let t8020 = t2474 * t1;
    (t8008, t8009, t8011, t8013, t8015, t8017, t8019, t8020)
}
