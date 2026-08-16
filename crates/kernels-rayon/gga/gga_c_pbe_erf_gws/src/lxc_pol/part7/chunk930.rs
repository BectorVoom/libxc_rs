//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 930/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk930(t1820: f64, t1866: f64, t1885: f64, t5307: f64, t17339: f64, t1888: f64, t5015: f64, t5312: f64, t17354: f64, t17359: f64, t17362: f64, t17364: f64, t17368: f64, t17372: f64, t17376: f64, t17378: f64) -> (f64, f64, f64, f64) {
    let t17382 = 8.0_f64 / 5.0_f64 * t1820 * t1885 * t5307 * t1866;
    let t17384 = 16.0_f64 / 5.0_f64 * t17339 * t1888;
    let t17386 = 16.0_f64 / 5.0_f64 * t5312 * t5015;
    let t17387 = t17354 - t17359 - t17362 + t17364 - t17368 + t17372 - t17376 + t17378 - t17382 - t17384 - t17386;
    (t17382, t17384, t17386, t17387)
}
