//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 793/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk793(t311: f64, t9471: f64, t1944: f64, t315: f64, t3271: f64, t871: f64, t1018: f64, t787: f64, t1026: f64, t2675: f64, t2679: f64, t2682: f64, t3348: f64) -> (f64, f64, f64, f64) {
    let t9472 = t311 * t9471;
    let t9473 = t1944 * t315;
    let t9474 = t9472 * t9473;
    let t9476 = t871 * t3271;
    let t9477 = t1018 * t787;
    let t9478 = t9476 * t9477;
    let t9480 = t2675 * t1026;
    let t9481 = t9480 * t2679;
    let t9483 = t3348 * t2682;
    (t9474, t9478, t9481, t9483)
}
