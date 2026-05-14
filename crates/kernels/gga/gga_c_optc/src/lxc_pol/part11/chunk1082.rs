//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1082/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1082<F: Float>(t19: F, t54760: F, t15889: F, t4380: F, t15597: F, t4444: F, t15843: F, t4450: F, t12079: F, t18042: F, t4435: F, t1179: F, t54391: F, t15828: F, t1162: F, t17903: F, t2367: F) -> (F, F, F, F, F, F, F, F) {
    let t54959 = t54760 * t19;
    let t54989 = t15889 * t4380;
    let t54999 = t4444 * t15597;
    let t55001 = t4450 * t15843;
    let t55004 = t4435 * t12079 * t18042;
    let t55011 = t1179 * t54391;
    let t55021 = t4450 * t15828;
    let t55024 = t1162 * t2367 * t17903;
    (t54959, t54989, t54999, t55001, t55004, t55011, t55021, t55024)
}
