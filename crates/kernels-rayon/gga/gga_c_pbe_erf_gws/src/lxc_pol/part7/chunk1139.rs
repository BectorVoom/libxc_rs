//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1139/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1139(t20412: f64, t6648: f64, t2105: f64, t343: f64, t874: f64, t2271: f64, t6643: f64, t822: f64, t2118: f64, t2382: f64, t6491: f64, t860: f64) -> (f64, f64, f64, f64) {
    let t20414 = t20412 * t6648 / 8.0_f64;
    let t20416 = t2105 * t874 * t343;
    let t20421 = t2271 * t6643;
    let t20422 = t822 * t20421;
    let t20424 = t20422 * t6648 / 8.0_f64;
    let t20428 = t2382 * t2118 * t6491 * t860 / 24.0_f64;
    (t20414, t20416, t20424, t20428)
}
