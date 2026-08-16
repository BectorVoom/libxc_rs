//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 880/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk880(t3242: f64, t6627: f64, t2319: f64, t3295: f64, t1105: f64, t2264: f64, t899: f64, t923: f64, t3249: f64, t6636: f64, t6684: f64, t2323: f64, t3279: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9598 = 7.0_f64 / 288.0_f64 * t6627 * t3242;
    let t9601 = 7.0_f64 / 1152.0_f64 * t2319 * t3295;
    let t9607 = t1105 * param_a_c;
    let t9630 = t899 * t2264 * t923;
    let t9632 = 7.0_f64 / 384.0_f64 * t9630 * t3249;
    let t9637 = t6684 * t6636;
    let t9645 = 35.0_f64 / 576.0_f64 * t2323 * t3279;
    (t9598, t9601, t9607, t9632, t9637, t9645)
}
