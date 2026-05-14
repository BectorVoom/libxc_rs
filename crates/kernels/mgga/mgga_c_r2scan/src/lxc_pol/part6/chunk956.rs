//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 956/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk956<F: Float>(t471: F, t7088: F, t97: F, t7007: F, t86: F, t2484: F, t406: F, t410: F, t166: F, t2483: F, t607: F, t1783: F, t898: F, t1422: F, t899: F, t1416: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7090 = t97 * t471 * t7088;
    let t7091 = 3.0 * t7090;
    let t7092 = t7007 * t86;
    let t7093 = 0.19751673498613801407e-1 * t7092;
    let t7094 = t406 * t2484;
    let t7095 = 8.0 * t7094;
    let t7096 = t410 * t2484;
    let t7097 = 8.0 * t7096;
    let t7098 = t7007 * t166;
    let t7101 = t2483 * t607;
    let t7104 = t898 * t1783;
    let t7107 = t1422 * t899;
    let t7108 = 32.0 * t7107;
    let t7109 = t1416 * t899;
    (t7090, t7091, t7092, t7093, t7094, t7095, t7096, t7097, t7098, t7101, t7104, t7107, t7108, t7109)
}
