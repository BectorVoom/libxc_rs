//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1120/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1120<F: Float>(t3155: F, t831: F, t1395: F, t1531: F, t5077: F, t5086: F, t177: F, t2911: F, t12529: F, t12547: F, t2918: F, t5138: F) -> (F, F, F, F) {
    let t13294 = t831 * t3155;
    let t13295 = t13294 / F::new(45.0);
    let t13296 = t1395 * t1531;
    let t13299 = F::new(4.0) / F::new(15.0) * t5077 * t13296 * t5086;
    let t13300 = t177 * t2911;
    let t13303 = F::new(8.0) / F::new(27.0) * t12529 * t13300 * t12547;
    let t13304 = t177 * t2918;
    let t13307 = F::new(2.0) / F::new(3.0) * t5138 * t13304 * t12547;
    (t13295, t13299, t13303, t13307)
}
