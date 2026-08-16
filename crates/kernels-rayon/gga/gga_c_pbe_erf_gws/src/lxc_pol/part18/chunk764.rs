//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 764/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk764(t1917: f64, t528: f64, t220: f64, t2735: f64, t211: f64, t1729: f64, t586: f64, t1791: f64, t642: f64, t1630: f64, t219: f64, t174: f64, t177: f64, t838: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4876 = 0.9973633333333333333e-1_f64 * t528 * t1917;
    let t4908 = t2735 * t220;
    let t4910 = 16.0_f64 / 405.0_f64 * t211 * t4908;
    let t4913 = t1729 * t586;
    let t4927 = t642 * t1791;
    let t4934 = t1630 * t219;
    let t4939 = t174 * t838 * t177;
    (t4876, t4910, t4913, t4927, t4934, t4939)
}
