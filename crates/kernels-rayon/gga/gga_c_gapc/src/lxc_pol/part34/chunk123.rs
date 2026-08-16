//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 123/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk123(t68: f64, t69: f64, t62: f64, t1: f64, t348: f64, t65: f64, t352: f64, t354: f64, t14: f64, t351: f64) -> (f64, f64, f64, f64, f64) {
    let t391 = 1.0_f64 / t69 / t68;
    let t392 = t62 * t391;
    let t394 = t348 * t65 * t1;
    let t399 = -0.66066666666666666667e-2_f64 * t352 - 0.41275e-2_f64 * t354;
    let t402 = -t394 * t351 / 12.0_f64 + t14 * t399 / 2.0_f64;
    (t391, t392, t394, t399, t402)
}
