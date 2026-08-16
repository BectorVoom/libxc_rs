//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1110/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1110(t35230: f64, t4393: f64, t8511: f64, t4414: f64, t7822: f64, t1181: f64, t30327: f64, t4358: f64, t599: f64, t30861: f64, t8649: f64, t4316: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35231 = 0.17149607247227894789e-2_f64 * t35230;
    let t35232 = t8511 * t4393;
    let t35234 = t7822 * t4414;
    let t35238 = t30327 * t1181 * t599 * t4358;
    let t35240 = t30861 * t8649;
    let t35242 = t7822 * t4316;
    (t35231, t35232, t35234, t35238, t35240, t35242)
}
