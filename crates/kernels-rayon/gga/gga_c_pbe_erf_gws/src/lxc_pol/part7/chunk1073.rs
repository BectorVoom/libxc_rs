//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1073/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1073(t19288: f64, t5761: f64, t1557: f64, t19284: f64, t19292: f64, t1: f64, t1533: f64, t392: f64, t485: f64, t497: f64, t501: f64, t5837: f64) -> (f64, f64, f64, f64, f64) {
    let t19320 = t5761 * t19288;
    let t19322 = t1557 * t19284;
    let t19324 = t1557 * t19292;
    let t19336 = t1533 * t1 * t392;
    let t19337 = t485 * t497 * t19336;
    let t19338 = 0.116921e2_f64 * t19337;
    let t19340 = t501 * t5837 * t19336;
    (t19320, t19322, t19324, t19338, t19340)
}
