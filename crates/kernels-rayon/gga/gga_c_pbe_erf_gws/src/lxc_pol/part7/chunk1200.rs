//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1200/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1200(t19505: f64, t2210: f64, t858: f64, t884: f64, t6322: f64, t6332: f64, t2145: f64, t6106: f64, t2150: f64, t2387: f64, t6710: f64, t6352: f64, t6416: f64) -> (f64, f64, f64, f64, f64) {
    let t21336 = 3.0_f64 / 16.0_f64 * t884 * t2210 * t858 * t19505;
    let t21337 = t6322 * t6332;
    let t21338 = 7.0_f64 / 12.0_f64 * t21337;
    let t21339 = t6106 * t2145;
    let t21341 = t21339 * t2150 / 12.0_f64;
    let t21346 = t2387 * t6710;
    let t21348 = t21346 * t2150 / 6.0_f64;
    let t21350 = t6416 * t6352;
    (t21336, t21338, t21341, t21348, t21350)
}
