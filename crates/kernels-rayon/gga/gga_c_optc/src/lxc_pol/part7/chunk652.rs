//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 652/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk652(t130: f64, t635: f64, t140: f64, t2086: f64, t106: f64, t145: f64, t146: f64, t692: f64, t112: f64) -> (f64, f64, f64, f64, f64) {
    let t3439 = t130 * t635;
    let t3440 = t2086 * t140;
    let t3461 = t106 * t145;
    let t3466 = t146 * t692;
    let t3467 = t3466 * t112;
    (t3439, t3440, t3461, t3466, t3467)
}
