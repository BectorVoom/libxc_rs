//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2665/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2665(t12550: f64, t12557: f64, t12725: f64, t12734: f64, t12813: f64, t1459: f64, t1774: f64, t19451: f64, t19456: f64, t20109: f64, t2314: f64, t2323: f64, t26114: f64, t4028: f64, t4073: f64, t4077: f64, t45632: f64, t5460: f64, t5494: f64, t55934: f64, t55969: f64, t574: f64, t652: f64, t7458: f64, t9348: f64) -> f64 {
    let t55998 = -4.0_f64 * t12813 * t1774 * t652 - 8.0_f64 * t12550 * t4028 - 8.0_f64 * t12550 * t7458 - 4.0_f64 * t12557 * t4028 - 8.0_f64 * t12725 * t4073 - 8.0_f64 * t12725 * t4077 - 8.0_f64 * t12734 * t5460 - 4.0_f64 * t1459 * t45632 - 8.0_f64 * t1459 * t55934 - 4.0_f64 * t19451 * t2323 - 8.0_f64 * t19456 * t4073 - 8.0_f64 * t20109 * t2314 - 8.0_f64 * t26114 * t4073 - 4.0_f64 * t5460 * t9348 - 2.0_f64 * t5494 * t9348 + t55969 * t574;
    t55998
}
