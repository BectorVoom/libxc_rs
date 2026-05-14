//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 755/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk755<F: Float>(t38: F, t5760: F, t370: F, t4394: F, t1234: F, t2229: F, t365: F, t110: F, t30: F, t342: F, t2233: F, t1227: F, t780: F, t348: F, t776: F, t2217: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5762 = 2.923025 * t38 * t5760;
    let t5763 = t370 * t4394;
    let t5766 = t2229 * t1234;
    let t5770 = t365 * t2229;
    let t5772 = t30 * t110 * t342;
    let t5774 = 5.87616 * t5770 * t5772;
    let t5775 = t2233 * t342;
    let t5779 = t780 * t1227;
    let t5783 = t348 * t776;
    let t5785 = 1.9486833333333333 * t5783 * t5772;
    let t5787 = t360 * t110 * t2217;
    (t5762, t5763, t5766, t5770, t5772, t5774, t5775, t5779, t5783, t5785, t5787)
}
