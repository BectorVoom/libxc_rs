//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1062/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1062(t6748: f64, t9529: f64, t6745: f64, t539: f64, t9521: f64, t1245: f64, t6322: f64, t6326: f64, t3386: f64, t6617: f64, t6636: f64, t1294: f64, t23017: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29348 = t9529 * t6748;
    let t29350 = t9529 * t6745;
    let t29352 = t539 * t9521;
    let t29354 = t6322 * t1245;
    let t29356 = t6326 * t1245;
    let t29365 = t3386 * t6617;
    let t29367 = t3386 * t6636;
    let t29441 = t23017 * t1294;
    (t29348, t29350, t29352, t29354, t29356, t29365, t29367, t29441)
}
