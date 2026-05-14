//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1260/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1260<F: Float>(t123: F, t566: F, t7113: F, t4454: F, t868: F, t1808: F, t2281: F, t18057: F, t199: F, t1152: F, t2422: F, t1200: F, t125: F, t15116: F, t18988: F, t18995: F, t2285: F, t2415: F, t7117: F) -> (F,) {
    let t18998 = t123 * t7113 * t566;
    let t19004 = t123 * t4454 * t868;
    let t19007 = t123 * t2281 * t1808;
    let t19017 = t123 * t18057 * t199;
    let t19020 = t123 * t1152 * t2422;
    let t19022 = -0.28298369577492777 * t18988 - 0.031835665774679375 * t123 * t2415 * t1200 + 0.10611888591559791 * t18995 + 0.10611888591559791 * t18998 - 0.1273426630987175 * t123 * t2285 * t1808 + 0.21223777183119583 * t19004 + 0.21223777183119583 * t19007 - 0.031835665774679375 * t123 * t125 * t15116 * t199 - 0.06367133154935875 * t123 * t7117 * t566 - 0.14149184788746388 * t19017 - 0.14149184788746388 * t19020;
    (t19022,)
}
