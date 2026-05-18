//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1195/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1195<F: Float>(t12581: F, t15983: F, t15986: F, t4492: F, t19: F, t54760: F, t15889: F, t4380: F, t15597: F, t4444: F, t15843: F, t4450: F) -> (F, F, F, F, F, F) {
    let t54944 = t12581 * t15983;
    let t54947 = t4492 * t15986;
    let t54959 = t54760 * t19;
    let t54989 = t15889 * t4380;
    let t54999 = t4444 * t15597;
    let t55001 = t4450 * t15843;
    (t54944, t54947, t54959, t54989, t54999, t55001)
}
