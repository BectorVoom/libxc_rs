//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 93/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk93(t277: f64, t40: f64, t243: f64, t85: f64, t1: f64, t60: f64, t132: f64, t203: f64, t84: f64) -> (f64, f64, f64, f64) {
    let t278 = t40 * t277;
    let t279 = t243 * t85;
    let t280 = 0.19751673498613801407e-1_f64 * t279;
    let t281 = t60 * t1;
    let t283 = t203 * t132 * t84;
    (t278, t280, t281, t283)
}
