//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1158/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1158<F: Float>(t10335: F, t10337: F, t10339: F, t10393: F, t10343: F, t10346: F, t10348: F, t10350: F, t10353: F, t10356: F, t10358: F, t10362: F) -> (F, F, F, F, F) {
    let t13822 = F::new(8.0) / F::new(405.0) * t10335;
    let t13823 = F::new(4.0) / F::new(45.0) * t10337;
    let t13824 = F::new(4.0) / F::new(135.0) * t10339;
    let t13829 = F::new(4.0) / F::new(45.0) * t10393;
    let t13830 = t13822 - t13823 + t13824 + t10343 + F::new(0.36466666666666664) * t10346 - F::new(2.0) / F::new(9.0) * t10348 - F::new(2.0) / F::new(3.0) * t10350 - F::new(0.040518518518518516) * t10353 - t10356 - t10358 + t10362 + t13829;
    (t13822, t13823, t13824, t13829, t13830)
}
