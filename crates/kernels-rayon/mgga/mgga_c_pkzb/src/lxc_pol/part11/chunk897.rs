//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 897/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk897(t201: f64, t9742: f64, t199: f64, t3719: f64, t967: f64, t1162: f64, t3298: f64, t3147: f64, t3157: f64, t1217: f64, t8028: f64, t3153: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9743 = t201 * t9742;
    let t9744 = t199 * t9743;
    let t9746 = t3719 * t967;
    let t9748 = t1162 * t3298;
    let t9751 = 0.11696447245269292414e1_f64 * t3147 * t3157;
    let t9753 = 0.11696447245269292414e1_f64 * t8028 * t1217;
    let t9755 = 0.23392894490538584828e1_f64 * t3147 * t3153;
    (t9744, t9746, t9748, t9751, t9753, t9755)
}
