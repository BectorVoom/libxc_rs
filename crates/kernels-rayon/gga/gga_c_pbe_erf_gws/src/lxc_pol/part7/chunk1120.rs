//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1120/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1120(t19777: f64, t4390: f64, t2271: f64, t4383: f64, t822: f64, t19745: f64, t2306: f64, t3074: f64, t19751: f64, t2118: f64, t2382: f64, t4384: f64) -> (f64, f64, f64, f64, f64) {
    let t20110 = t19777 * t4390;
    let t20112 = t2271 * t4383;
    let t20113 = t822 * t20112;
    let t20117 = t3074 * t2306 * t19745;
    let t20121 = t2382 * t2118 * t19751;
    let t20124 = t2382 * t4384;
    (t20110, t20113, t20117, t20121, t20124)
}
