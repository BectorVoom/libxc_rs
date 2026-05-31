//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 649/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk649<F: Float>(t3226: F, t500: F, t1447: F, t1455: F, t1467: F, t1461: F, t511: F, t1414: F, t164: F) -> (F, F, F, F, F) {
    let t3227 = t3226 * t500;
    let t3231 = t1447 * t1455;
    let t3233 = t1447 * t1467;
    let t3238 = t1461 * t511;
    let t3247 = F::cast_from(1.0_f64) / t164 / t1414;
    (t3227, t3231, t3233, t3238, t3247)
}
