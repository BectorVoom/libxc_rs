//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1137/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1137(t20388: f64, t2121: f64, t337: f64, t6645: f64, t2387: f64, t6187: f64, t2138: f64, t6293: f64, t6402: f64, t20366: f64, t20367: f64, t20370: f64, t20371: f64, t20377: f64, t20381: f64, t20385: f64, t20386: f64, t2312: f64, t6609: f64, t9482: f64) -> (f64, f64, f64) {
    let t20390 = t2121 * t337 * t20388;
    let t20392 = t6645 * t20390 / 4.0_f64;
    let t20393 = t2387 * t6187;
    let t20395 = t20393 * t2138 / 12.0_f64;
    let t20396 = t6402 * t6293;
    let t20398 = -t20366 + 7.0_f64 / 48.0_f64 * t20367 - t20370 - t2312 * t9482 * t6609 * t20371 / 24.0_f64 + t20377 + t20381 + t20385 + 7.0_f64 / 96.0_f64 * t20386 + t20392 - t20395 + 7.0_f64 / 96.0_f64 * t20396;
    (t20392, t20395, t20398)
}
