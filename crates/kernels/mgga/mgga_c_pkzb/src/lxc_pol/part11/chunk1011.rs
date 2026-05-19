//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1011/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1011<F: Float>(t11133: F, t326: F, t427: F, t1270: F, t3719: F, t1162: F, t3949: F, t3152: F, t3820: F, t898: F, t10523: F, t6097: F) -> (F, F, F, F, F, F, F) {
    let t11134 = t326 * t11133;
    let t11135 = t11134 * t427;
    let t11136 = F::new(0.57375e0) * t11135;
    let t11137 = t3719 * t1270;
    let t11138 = F::new(0.4303125e0) * t11137;
    let t11139 = t1162 * t3949;
    let t11140 = F::new(0.1434375e0) * t11139;
    let t11141 = t3152 * t3820;
    let t11143 = F::cast_from(0.35089341735807877242e1_f64) * t898 * t11141;
    let t11146 = t6097 * t10523;
    (t11134, t11136, t11138, t11140, t11141, t11143, t11146)
}
