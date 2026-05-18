//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1202/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1202<F: Float>(t54760: F, t8915: F, t3133: F, t55037: F, t9128: F, t12869: F, t18065: F, t4457: F, t15831: F, t4450: F, t1157: F, t17969: F) -> (F, F, F, F, F) {
    let t55487 = t54760 * t8915;
    let t55493 = t9128 * t55037 * t3133;
    let t55496 = t4457 * t12869 * t18065;
    let t55498 = t4450 * t15831;
    let t55550 = t17969 * t1157;
    (t55487, t55493, t55496, t55498, t55550)
}
