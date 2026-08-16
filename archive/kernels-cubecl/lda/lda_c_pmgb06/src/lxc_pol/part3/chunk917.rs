//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 917/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk917<F: Float>(t122: F, t227: F, t8088: F, t107: F, t1126: F, t1180: F, t391: F, t4209: F, t199: F, t2778: F, t4169: F, t569: F) -> (F, F, F, F, F) {
    let t10472 = F::cast_from(0.9079060239445599_f64) * t122 * t8088 * t227;
    let t10474 = t107 * t1180 * t1126;
    let t10476 = t391 * t4209;
    let t10479 = F::cast_from(2.0103076928521055_f64) * t2778 * t199;
    let t10481 = t122 * t569 * t4169;
    (t10472, t10474, t10476, t10479, t10481)
}
