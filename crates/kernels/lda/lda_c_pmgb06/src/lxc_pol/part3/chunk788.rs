//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 788/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk788<F: Float>(t962: F, t977: F, t3741: F, t696: F, t8599: F, t3760: F, t971: F, t138: F, t28: F, t4238: F, t8333: F, t3679: F, t642: F, t1147: F, t934: F, t940: F) -> (F, F, F, F, F, F, F) {
    let t8688 = 1.0 / t962 / t977;
    let t8692 = 12304.822629859687 * t696 * t8688 * t8599 * t3741;
    let t8693 = t971 * t3760;
    let t8697 = t8333 * t28 * t4238 * t138;
    let t8699 = t3679 * t642;
    let t8701 = t934 * t1147;
    let t8702 = t940 * t8701;
    (t8688, t8692, t8693, t8697, t8699, t8701, t8702)
}
