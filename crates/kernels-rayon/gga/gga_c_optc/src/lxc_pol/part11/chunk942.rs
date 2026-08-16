//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 942/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk942(t4095: f64, t5133: f64, t4111: f64, t1045: f64, t17380: f64, t17340: f64, t2869: f64, t25: f64) -> (f64, f64, f64, f64, f64) {
    let t17399 = t4095 * t5133;
    let t17401 = t4111 * t5133;
    let t17403 = t1045 * t17380;
    let t17405 = t2869 * t17340;
    let t17406 = t25 * t17405;
    (t17399, t17401, t17403, t17405, t17406)
}
