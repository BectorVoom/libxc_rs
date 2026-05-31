//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1022/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1022<F: Float>(t1512: F, t1548: F, t2857: F, t432: F, t1441: F, t3213: F, t1431: F, t1179: F, t161: F, t165: F, t177: F, t1462: F, t1600: F) -> (F, F, F, F, F, F) {
    let t10087 = t1512 * t1548;
    let t10089 = t432 * t2857;
    let t10099 = t3213 * t1441;
    let t10109 = t3213 * t1431;
    let t10134 = F::cast_from(28.0_f64) / F::cast_from(1215.0_f64) * t161 * t1179 * t165 * t177;
    let t10139 = t1462 * t1600;
    (t10087, t10089, t10099, t10109, t10134, t10139)
}
