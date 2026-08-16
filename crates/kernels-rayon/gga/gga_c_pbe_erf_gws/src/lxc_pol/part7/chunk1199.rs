//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1199/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1199(t20469: f64, t3065: f64, t858: f64, t6678: f64, t2285: f64, t6455: f64, t2206: f64, t6661: f64, t19: f64, t2298: f64, t56: f64, t21011: f64, t884: f64) -> (f64, f64, f64, f64) {
    let t21316 = t3065 * t858 * t20469;
    let t21318 = t6678 * t21316 / 16.0_f64;
    let t21319 = t6455 * t2285;
    let t21325 = t2206 * t6661;
    let t21326 = 7.0_f64 / 3.0_f64 * t21325;
    let t21328 = t56 * t2298 * t19;
    let t21332 = 5.0_f64 / 4.0_f64 * t884 * t21328 * t858 * t21011;
    (t21318, t21319, t21326, t21332)
}
