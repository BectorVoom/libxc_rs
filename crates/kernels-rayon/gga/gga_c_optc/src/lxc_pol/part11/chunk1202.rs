//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1202/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1202(t54760: f64, t8915: f64, t3133: f64, t55037: f64, t9128: f64, t12869: f64, t18065: f64, t4457: f64, t15831: f64, t4450: f64, t1157: f64, t17969: f64) -> (f64, f64, f64, f64, f64) {
    let t55487 = t54760 * t8915;
    let t55493 = t9128 * t55037 * t3133;
    let t55496 = t4457 * t12869 * t18065;
    let t55498 = t4450 * t15831;
    let t55550 = t17969 * t1157;
    (t55487, t55493, t55496, t55498, t55550)
}
