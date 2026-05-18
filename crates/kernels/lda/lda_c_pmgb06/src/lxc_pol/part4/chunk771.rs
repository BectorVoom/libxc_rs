//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 771/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk771<F: Float>(t132: F, t5115: F, t1592: F, t813: F, t1594: F, t137: F, t1604: F, t831: F, t1392: F, t802: F, t1631: F, t3051: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5117 = F::new(2.0) / F::new(45.0) * t132 * t5115;
    let t5118 = t813 * t1592;
    let t5119 = t5118 * t1594;
    let t5120 = t137 * t5119;
    let t5122 = t132 * t5120 / F::new(15.0);
    let t5124 = t831 * t1604 / F::new(15.0);
    let t5126 = F::new(2.0) / F::new(45.0) * t802 * t1392;
    let t5128 = t802 * t1631 / F::new(30.0);
    let t5129 = t3051 / F::new(45.0);
    (t5117, t5118, t5119, t5120, t5122, t5124, t5126, t5128, t5129)
}
