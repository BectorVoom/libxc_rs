//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1253/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1253<F: Float>(t12092: F, t153: F, t1859: F, t439: F, t4659: F, t6498: F, t13715: F, t4645: F, t2010: F, t4655: F, t1444: F, t6504: F) -> (F, F, F, F, F) {
    let t16487 = F::new(4.0) / F::new(27.0) * t439 * t12092 * t153 * t1859;
    let t16490 = F::new(2.0) / F::new(27.0) * t439 * t6498 * t4659;
    let t16491 = t13715 * t153;
    let t16494 = F::new(16.0) / F::new(81.0) * t439 * t16491 * t4645;
    let t16497 = F::new(8.0) / F::new(27.0) * t2010 * t6498 * t4655;
    let t16499 = F::new(4.0) / F::new(9.0) * t1444 * t6504;
    (t16487, t16490, t16494, t16497, t16499)
}
