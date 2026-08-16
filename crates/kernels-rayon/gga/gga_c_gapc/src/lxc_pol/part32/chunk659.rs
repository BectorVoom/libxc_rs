//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 659/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk659(t1881: f64, t190: f64, t1033: f64, t198: f64, t5: f64, t681: f64, t19: f64, t147: f64, t203: f64, t144: f64, t1: f64, t457: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5294 = t1881 * t190;
    let t5296 = t1033 * t198;
    let t5298 = t681 * t5;
    let t5311 = t5 * t19;
    let t5312 = t5311 * t147;
    let t5319 = t203 * t5;
    let t5325 = t1033 * t144;
    let t5390 = t457 * t1;
    (t5294, t5296, t5298, t5312, t5319, t5325, t5390)
}
