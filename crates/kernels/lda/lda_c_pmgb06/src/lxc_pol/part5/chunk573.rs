//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 573/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk573<F: Float>(t3697: F, t633: F, t622: F, t959: F, t971: F, t681: F, t964: F) -> (F, F, F, F) {
    let t3698 = t3697 * t633;
    let t3700 = F::new(1.0) * t622 * t3698;
    let t3701 = t971 * t959;
    let t3703 = t964 * t681;
    (t3698, t3700, t3701, t3703)
}
