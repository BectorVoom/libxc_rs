//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 895/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk895<F: Float>(t12695: F, t519: F, t6442: F, t108: F, t209: F, t12781: F, t1325: F, t6432: F, t504: F, t6566: F, t10463: F, t2392: F, t6229: F, t3863: F, t571: F, t6286: F) -> (F, F, F, F, F, F, F) {
    let t16140 = t519 * t12695 * t6442;
    let t16144 = t209 * t108;
    let t16159 = t1325 * t12781 * t6432;
    let t16209 = t6566 * t504;
    let t16221 = t1325 * t10463 * t2392;
    let t16224 = t1325 * t12695 * t6229;
    let t16232 = t571 * t3863 * t6286;
    (t16140, t16144, t16159, t16209, t16221, t16224, t16232)
}
