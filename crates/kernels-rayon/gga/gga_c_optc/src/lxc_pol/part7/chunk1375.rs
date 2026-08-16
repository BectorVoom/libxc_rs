//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1375/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1375(t1135: f64, t509: f64, t3234: f64, t3237: f64, t1179: f64, t2586: f64, t9201: f64, t3236: f64, t8429: f64, t12068: f64, t9069: f64, t3105: f64, t430: f64) -> (f64, f64, f64, f64, f64) {
    let t27351 = t509 * t1135;
    let t27353 = t3234 * t27351 * t3237;
    let t27356 = t1179 * t2586 * t9201;
    let t27358 = t8429 * t3236;
    let t27363 = t3234 * t12068 * t9069;
    let t27365 = t430 * t3105;
    (t27353, t27356, t27358, t27363, t27365)
}
