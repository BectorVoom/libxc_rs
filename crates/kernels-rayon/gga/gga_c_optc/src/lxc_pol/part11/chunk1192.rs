//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1192/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1192(t1133: f64, t54621: f64, t1146: f64, t17947: f64, t4356: f64, t5101: f64, t1113: f64, t17697: f64, t8915: f64, t1135: f64, t5096: f64, t1148: f64, t17927: f64, t911: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54622 = t1133 * t54621;
    let t54641 = t17947 * t1146;
    let t54716 = t4356 * t5101;
    let t54753 = t1113 * t17697;
    let t54754 = t54753 * t8915;
    let t54760 = t1135 * t17697;
    let t54777 = t4356 * t5096;
    let t54789 = t1148 * t17927 * t911;
    (t54622, t54641, t54716, t54753, t54754, t54760, t54777, t54789)
}
