//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1126/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1126(t6180: f64, t6188: f64, t2189: f64, t343: f64, t745: f64, t2121: f64, t2122: f64, t337: f64, t2382: f64, t6566: f64, t20189: f64, t2137: f64) -> (f64, f64, f64, f64) {
    let t20234 = t6188 * t6180 / 16.0_f64;
    let t20236 = t745 * t2189 * t343;
    let t20244 = t2121 * t337 * t2122 * t745;
    let t20246 = 7.0_f64 / 48.0_f64 * t2382 * t6566 * t20244;
    let t20247 = t20189 * t2137;
    (t20234, t20236, t20246, t20247)
}
