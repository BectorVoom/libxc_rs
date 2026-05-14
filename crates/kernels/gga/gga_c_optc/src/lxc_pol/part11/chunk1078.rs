//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1078/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1078<F: Float>(t15335: F, t4369: F, t15597: F, t140: F, t17648: F, t464: F, t871: F, t17885: F, t309: F, t441: F, t1128: F, t17709: F, t8921: F, t17713: F, t2586: F, t1133: F) -> (F, F, F, F, F, F, F, F) {
    let t54541 = t4369 * t15335;
    let t54589 = t4369 * t15597;
    let t54596 = t464 * t17648 * t871 * t140;
    let t54599 = t17885 * t309;
    let t54600 = t441 * t54599;
    let t54613 = t8921 * t1128 * t17709;
    let t54615 = t2586 * t17713;
    let t54616 = t1133 * t54615;
    (t54541, t54589, t54596, t54599, t54600, t54613, t54615, t54616)
}
