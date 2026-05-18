//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 595/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk595<F: Float>(t3226: F, t500: F, t1444: F, t1467: F, t1447: F, t1455: F, t1450: F, t1454: F, t493: F, t1461: F, t511: F, t1466: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3227 = t3226 * t500;
    let t3228 = F::new(4.0) / F::new(45.0) * t3227;
    let t3230 = t1444 * t1467 / F::new(9.0);
    let t3231 = t1447 * t1455;
    let t3232 = F::new(2.0) / F::new(45.0) * t3231;
    let t3233 = t1447 * t1467;
    let t3234 = F::new(2.0) / F::new(27.0) * t3233;
    let t3235 = t1450 * t1454;
    let t3237 = t493 * t3235 / F::new(15.0);
    let t3238 = t1461 * t511;
    let t3239 = t3238 * t1466;
    (t3227, t3228, t3230, t3231, t3232, t3233, t3234, t3235, t3237, t3238, t3239)
}
