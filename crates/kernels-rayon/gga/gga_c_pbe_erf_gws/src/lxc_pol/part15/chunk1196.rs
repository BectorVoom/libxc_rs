//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1196/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1196(t3969: f64, t916: f64, t2250: f64, t14024: f64, t2129: f64, t2153: f64, t899: f64, t923: f64, t2348: f64, t56: f64, t837: f64, t863: f64, t911: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51350 = t3969 * t916;
    let t51351 = t2250 * t51350;
    let t51358 = t2129 * t14024;
    let t51371 = t899 * t2153 * t923;
    let t51372 = t51371 * t2348;
    let t51382 = t863 * t911 * t837 * t56;
    (t51350, t51351, t51358, t51371, t51372, t51382)
}
