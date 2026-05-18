//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1065/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1065<F: Float>(t1444: F, t4762: F, t1989: F, t3223: F, t4761: F, t493: F, t5179: F, t9596: F, t9598: F, t9601: F, t1980: F, t883: F) -> (F, F, F, F, F, F, F) {
    let t12648 = F::new(3.0) / F::new(5.0) * t1444 * t4762;
    let t12649 = t3223 * t1989;
    let t12650 = F::new(2.0) / F::new(135.0) * t12649;
    let t12653 = F::new(3.0) / F::new(5.0) * t493 * t5179 * t4761;
    let t12654 = F::new(4.0) / F::new(135.0) * t9596;
    let t12655 = F::new(2.0) / F::new(45.0) * t9598;
    let t12656 = F::new(2.0) / F::new(45.0) * t9601;
    let t12657 = t883 * t1980;
    (t12648, t12650, t12653, t12654, t12655, t12656, t12657)
}
