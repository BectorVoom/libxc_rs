//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 786/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk786(t2173: f64, t6484: f64, t2113: f64, t2127: f64, t850: f64, t860: f64, t1452: f64, t339: f64, t851: f64, t6440: f64, t904: f64, t916: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6485 = t6484 * t2173;
    let t6486 = 7.0_f64 / 24.0_f64 * t6485;
    let t6488 = t850 * t2113 * t2127;
    let t6490 = t6488 * t860 / 48.0_f64;
    let t6491 = t1452 * t339;
    let t6493 = t850 * t851 * t6491;
    let t6495 = t6493 * t860 / 96.0_f64;
    let t6497 = t916 * t904 * t6440;
    (t6486, t6488, t6490, t6491, t6493, t6495, t6497)
}
