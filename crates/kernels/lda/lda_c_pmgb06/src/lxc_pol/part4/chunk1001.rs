//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1001/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1001<F: Float>(t188: F, t3023: F, t794: F, t117: F, t123: F, t2360: F, t740: F, t1147: F, t859: F, t2791: F, t795: F, t415: F, t5543: F, t1347: F, t1799: F, t315: F, t5689: F) -> (F, F, F, F, F, F, F) {
    let t14484 = t794 * t3023 * t188;
    let t14500 = t123 * t740 * t2360 * t117;
    let t14527 = t123 * t1147 * t859 * t117;
    let t14529 = t795 * t2791;
    let t14533 = t5543 * t415;
    let t14535 = t1799 * t1347;
    let t14539 = t123 * t315 * t5689 * t117;
    (t14484, t14500, t14527, t14529, t14533, t14535, t14539)
}
