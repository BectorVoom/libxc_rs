//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 600/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk600<F: Float>(t1139: F, t199: F, t566: F, t718: F, t26: F, t386: F, t1322: F, t73: F) -> (F, F, F, F) {
    let t4212 = t1139 * t199;
    let t4214 = t718 * t566;
    let t4230 = t26 * t386;
    let t4232 = t1322 * t73;
    (t4212, t4214, t4230, t4232)
}
