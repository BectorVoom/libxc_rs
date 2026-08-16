//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1219/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1219(t21105: f64, t823: f64, t850: f64, t852: f64, t860: f64, t21601: f64, t2168: f64, t8599: f64, t16463: f64, t333: f64, t56: f64, t338: f64, t348: f64) -> (f64, f64, f64, f64) {
    let t21632 = t850 * t21105 * t823 * t852 * t860 / 96.0_f64;
    let t21635 = 3.0_f64 / 4.0_f64 * t2168 * t8599 * t21601;
    let t21637 = t16463 * t56 * t333;
    let t21640 = 455.0_f64 / 243.0_f64 * t348 * t21637 * t338;
    (t21632, t21635, t21637, t21640)
}
