//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1190/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1190<F: Float>(t15355: F, t4333: F, t1111: F, t11885: F, t17344: F, t1128: F, t17699: F, t8960: F, t15335: F, t4369: F, t15597: F, t140: F, t17648: F, t464: F, t871: F) -> (F, F, F, F, F, F) {
    let t54520 = t15355 * t4333;
    let t54523 = t1111 * t11885 * t17344;
    let t54527 = t8960 * t1128 * t17699;
    let t54541 = t4369 * t15335;
    let t54589 = t4369 * t15597;
    let t54596 = t464 * t17648 * t871 * t140;
    (t54520, t54523, t54527, t54541, t54589, t54596)
}
