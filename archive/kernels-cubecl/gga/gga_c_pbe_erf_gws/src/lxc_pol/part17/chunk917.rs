//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 917/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk917<F: Float>(t4810: F, t4817: F, t2513: F, t409: F, t2515: F, t414: F, t1336: F, t960: F, t1396: F, t2840: F, t1392: F, t1: F, t2474: F) -> (F, F, F, F, F, F, F, F) {
    let t8008 = F::cast_from(0.48830813431341759842e-3_f64) * t4810;
    let t8009 = F::cast_from(0.18311555036753159941e-3_f64) * t4817;
    let t8010 = t409 * t2513;
    let t8011 = F::cast_from(8.0_f64) * t8010;
    let t8012 = t414 * t2515;
    let t8013 = F::cast_from(8.0_f64) * t8012;
    let t8014 = t1336 * t960;
    let t8015 = F::cast_from(12.0_f64) * t8014;
    let t8016 = t2840 * t1396;
    let t8017 = F::cast_from(0.58482233974552040708e0_f64) * t8016;
    let t8018 = t2840 * t1392;
    let t8019 = F::cast_from(0.17315755899375863299e2_f64) * t8018;
    let t8020 = t2474 * t1;
    (t8008, t8009, t8011, t8013, t8015, t8017, t8019, t8020)
}
