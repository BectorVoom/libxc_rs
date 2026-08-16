//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 658/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk658<F: Float>(t170: F, t3457: F, t117: F, t123: F, t550: F, t740: F, t1650: F, t315: F, t1135: F, t118: F, t103: F, t37: F) -> (F, F, F, F, F) {
    let t3458 = t170 * t3457;
    let t3474 = t123 * t740 * t550 * t117;
    let t3478 = t123 * t315 * t1650 * t117;
    let t3481 = F::cast_from(0.1890324433388467_f64) * t1135 * t118;
    let t3500 = F::cast_from(1.0_f64) / t37 / t103 / F::cast_from(4.0_f64);
    (t3458, t3474, t3478, t3481, t3500)
}
