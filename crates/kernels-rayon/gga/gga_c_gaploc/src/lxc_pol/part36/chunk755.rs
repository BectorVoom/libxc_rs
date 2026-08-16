//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 755/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk755(t20669: f64, t584: f64, t20687: f64, t1406: f64, t6582: f64, t9271: f64, t10530: f64, t6574: f64, t123: f64, t18313: f64, t197: f64, t3116: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31041 = t584 * t20669;
    let t31047 = t584 * t20687;
    let t31051 = t1406 * t6582;
    let t31054 = t1406 * t9271;
    let t31119 = t584 * t10530 * t6574;
    let t31120 = t18313 * t123;
    let t31139 = t197 * t3116;
    (t31041, t31047, t31051, t31054, t31119, t31120, t31139)
}
