//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1093/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1093<F: Float>(t1394: F, t15904: F, t1017: F, t541: F, t86: F, t2011: F, t4134: F, t4129: F, t4162: F, t3734: F, t5628: F, t1464: F, t1650: F, t4124: F, t4163: F, t12241: F) -> (F, F, F, F, F) {
    let t15905 = t1394 * t15904;
    let t15909 = t86 * t1017 * t541;
    let t15910 = t4134 * t2011;
    let t15911 = t15910 * t4129;
    let t15912 = t4162 * t15911;
    let t15913 = t15909 * t15912;
    let t15915 = t3734 * t5628;
    let t15916 = t1464 * t15915;
    let t15919 = t4163 * t1650 * t4124;
    let t15920 = t12241 * t15919;
    (t15905, t15909, t15913, t15916, t15920)
}
