//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1284/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1284(t21296: f64, t367: f64, t899: f64, t9427: f64, t3237: f64, t51371: f64, t3242: f64, t14011: f64, t9634: f64, t3232: f64, t4028: f64, t9103: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54279 = t899 * t21296 * t367;
    let t54280 = t54279 * t9427;
    let t54283 = t51371 * t3237;
    let t54285 = t51371 * t3242;
    let t54287 = t14011 * t9634;
    let t54289 = t51371 * t3232;
    let t54295 = t4028 * t9103;
    (t54280, t54283, t54285, t54287, t54289, t54295)
}
