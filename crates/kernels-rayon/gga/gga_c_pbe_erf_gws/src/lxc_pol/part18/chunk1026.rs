//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1026/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1026(t11478: f64, t2170: f64, t2171: f64, t2168: f64, t3180: f64, t9188: f64, t8956: f64, t3793: f64, t8949: f64, t11451: f64, t11458: f64, t11463: f64, t11466: f64, t11472: f64, t11477: f64, t2253: f64, t2343: f64, t8826: f64, t8835: f64, t8846: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11480 = t2170 * t11478 * t2171;
    let t11482 = t2168 * t11480 / 48.0_f64;
    let t11484 = t9188 * t3180 / 24.0_f64;
    let t11486 = t8956 * t3180 / 24.0_f64;
    let t11488 = t8949 * t3793 / 96.0_f64;
    let t11489 = t8826 - t2253 * t11451 / 384.0_f64 + t8835 - t11458 + t11463 + t2343 * t11466 / 384.0_f64 - t11472 + t11477 + t11482 - t11484 - t11486 - t8846 - t11488;
    (t11480, t11482, t11484, t11486, t11488, t11489)
}
