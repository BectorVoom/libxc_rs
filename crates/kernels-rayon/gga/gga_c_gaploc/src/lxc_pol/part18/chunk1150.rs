//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1150/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1150(t31045: f64, t20687: f64, t584: f64, t20561: f64, t20671: f64, t1406: f64, t6582: f64, t9268: f64, t9271: f64, t9274: f64, t1265: f64, t2487: f64, t9438: f64, t9448: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31046 = 0.51123901271894332901e1_f64 * t31045;
    let t31047 = t584 * t20687;
    let t31050 = 0.85206502119823888169e0_f64 * t31047 * t20671 * t20561;
    let t31051 = t1406 * t6582;
    let t31053 = 0.38342925953920749676e1_f64 * t31051 * t9268;
    let t31054 = t1406 * t9271;
    let t31056 = 0.23005755572352449806e1_f64 * t31054 * t9274;
    let t31065 = t2487 * t9438 * t9448 * t1265;
    (t31046, t31047, t31050, t31051, t31053, t31054, t31056, t31065)
}
