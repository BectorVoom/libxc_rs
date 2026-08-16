//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1196/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1196(t2302: f64, t6505: f64, t19859: f64, t6659: f64, t858: f64, t884: f64, t2206: f64, t6694: f64, t2192: f64, t6228: f64, t20726: f64, t6241: f64) -> (f64, f64, f64, f64, f64) {
    let t21269 = t6505 * t2302;
    let t21274 = 3.0_f64 / 2.0_f64 * t884 * t6659 * t858 * t19859;
    let t21279 = t2206 * t6694;
    let t21280 = 7.0_f64 / 36.0_f64 * t21279;
    let t21285 = t6228 * t2192;
    let t21286 = 35.0_f64 / 72.0_f64 * t21285;
    let t21287 = t20726 * t6241;
    (t21269, t21274, t21280, t21286, t21287)
}
