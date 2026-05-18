//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1132/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1132<F: Float>(t10190: F, t10196: F, t1966: F, t1967: F, t3441: F, t439: F, t1972: F, t3251: F, t835: F, t9370: F, t1977: F, t3198: F) -> (F, F, F, F, F, F) {
    let t13456 = F::new(2.0) / F::new(45.0) * t10190;
    let t13457 = F::new(2.0) / F::new(27.0) * t10196;
    let t13461 = t439 * t1966 * t1967 * t3441 / F::new(15.0);
    let t13463 = F::new(8.0) / F::new(81.0) * t1972 * t3251;
    let t13465 = t9370 * t835 / F::new(45.0);
    let t13467 = t3198 * t1977 / F::new(15.0);
    (t13456, t13457, t13461, t13463, t13465, t13467)
}
