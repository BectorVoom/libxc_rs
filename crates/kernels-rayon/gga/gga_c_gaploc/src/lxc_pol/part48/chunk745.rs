//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 745/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk745(t4779: f64, t584: f64, t9419: f64, t20669: f64, t20687: f64, t1406: f64, t6582: f64, t9271: f64, t10530: f64, t6574: f64, t6575: f64, t10215: f64, t203: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31037 = t584 * t4779 * t9419;
    let t31041 = t584 * t20669;
    let t31047 = t584 * t20687;
    let t31051 = t1406 * t6582;
    let t31054 = t1406 * t9271;
    let t31119 = t584 * t10530 * t6574;
    let t31356 = t1406 * t6575;
    let t31501 = t203 * t10215;
    (t31037, t31041, t31047, t31051, t31054, t31119, t31356, t31501)
}
