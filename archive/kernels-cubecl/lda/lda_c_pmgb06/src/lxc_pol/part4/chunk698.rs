//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 698/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk698<F: Float>(t566: F, t718: F, t199: F, t2813: F, t1329: F, t1200: F, t391: F, t26: F, t386: F, t329: F, t1322: F, t73: F) -> (F, F, F, F, F, F, F) {
    let t4214 = t718 * t566;
    let t4216 = t2813 * t199;
    let t4218 = t1329 * t566;
    let t4220 = t391 * t1200;
    let t4230 = t26 * t386;
    let t4231 = t329 * t4230;
    let t4232 = t1322 * t73;
    (t4214, t4216, t4218, t4220, t4230, t4231, t4232)
}
