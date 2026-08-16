//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1472/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1472<F: Float>(t123: F, t4429: F, t868: F, t199: F, t315: F, t6716: F, t566: F, t7113: F, t4454: F, t1808: F, t2281: F, t18057: F) -> (F, F, F, F, F, F) {
    let t18988 = t123 * t4429 * t868;
    let t18995 = t123 * t315 * t6716 * t199;
    let t18998 = t123 * t7113 * t566;
    let t19004 = t123 * t4454 * t868;
    let t19007 = t123 * t2281 * t1808;
    let t19017 = t123 * t18057 * t199;
    (t18988, t18995, t18998, t19004, t19007, t19017)
}
