//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 901/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk901<F: Float>(t123: F, t199: F, t315: F, t6716: F, t566: F, t7113: F, t4454: F, t868: F, t1808: F, t2281: F, t18057: F, t1152: F, t2422: F, t6939: F, t722: F, t2753: F, t754: F, t936: F, t97: F) -> (F, F, F, F, F, F, F, F) {
    let t18995 = t123 * t315 * t6716 * t199;
    let t18998 = t123 * t7113 * t566;
    let t19004 = t123 * t4454 * t868;
    let t19007 = t123 * t2281 * t1808;
    let t19017 = t123 * t18057 * t199;
    let t19020 = t123 * t1152 * t2422;
    let t19031 = t123 * t722 * t6939;
    let t19055 = t2753 * t754 * t97 * t936;
    (t18995, t18998, t19004, t19007, t19017, t19020, t19031, t19055)
}
