//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1198/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1198<F: Float>(t3119: F, t5101: F, t1179: F, t54621: F, t17855: F, t430: F, t15335: F, t4444: F, t1162: F, t17897: F, t2367: F, t12578: F, t16001: F) -> (F, F, F, F, F, F) {
    let t55145 = t3119 * t5101;
    let t55162 = t1179 * t54621;
    let t55164 = t430 * t17855;
    let t55176 = t4444 * t15335;
    let t55181 = t1162 * t2367 * t17897;
    let t55194 = t12578 * t16001;
    (t55145, t55162, t55164, t55176, t55181, t55194)
}
