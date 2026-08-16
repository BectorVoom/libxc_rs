//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 956/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk956(t3133: f64, t9129: f64, t1115: f64, t7274: f64, t1162: f64, t3128: f64, t8469: f64, t3245: f64, t9040: f64, t1113: f64, t24: f64) -> (f64, f64, f64, f64, f64) {
    let t9130 = t9129 * t3133;
    let t9133 = t7274 * t1115;
    let t9134 = t1162 * t9133;
    let t9136 = t8469 * t3128;
    let t9139 = t3245 * t9040;
    let t9142 = t24 * t1113;
    (t9130, t9134, t9136, t9139, t9142)
}
