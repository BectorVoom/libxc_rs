//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1042/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1042<F: Float>(t15422: F, t933: F, t116: F, t12769: F, t982: F, t979: F, t119: F, t3127: F, t140: F, t191: F, t1002: F, t3174: F) -> (F, F, F, F) {
    let t15423 = t15422 * t933;
    let t15426 = t116 * t12769;
    let t15427 = t982 * t15426;
    let t15428 = t979 * t15427;
    let t15430 = t119 * t3127;
    let t15432 = t140 * t15430 * t191;
    let t15434 = t1002 * t3174;
    (t15423, t15428, t15432, t15434)
}
