//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1190/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1190(t15355: f64, t4333: f64, t1111: f64, t11885: f64, t17344: f64, t1128: f64, t17699: f64, t8960: f64, t15335: f64, t4369: f64, t15597: f64, t140: f64, t17648: f64, t464: f64, t871: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54520 = t15355 * t4333;
    let t54523 = t1111 * t11885 * t17344;
    let t54527 = t8960 * t1128 * t17699;
    let t54541 = t4369 * t15335;
    let t54589 = t4369 * t15597;
    let t54596 = t464 * t17648 * t871 * t140;
    (t54520, t54523, t54527, t54541, t54589, t54596)
}
