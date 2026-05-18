//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 597/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk597<F: Float>(t170: F, t3247: F, t176: F, t2911: F, t2912: F, t493: F, t3115: F, t444: F, t442: F, t439: F, t135: F, t1531: F) -> (F, F, F, F, F, F, F, F) {
    let t3248 = t3247 * t170;
    let t3249 = t176 * t2911;
    let t3250 = t3249 * t2912;
    let t3251 = t3248 * t3250;
    let t3253 = F::new(8.0) / F::new(81.0) * t493 * t3251;
    let t3254 = t444 * t3115;
    let t3255 = t442 * t3254;
    let t3257 = t439 * t3255 / F::new(45.0);
    let t3259 = F::new(1.0) / t135 / t1531;
    (t3248, t3250, t3251, t3253, t3254, t3255, t3257, t3259)
}
