//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1224/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1224<F: Float>(t1972: F, t5180: F, t136: F, t1872: F, t1968: F, t439: F, t4762: F, t4608: F, t6550: F, t12075: F, t1423: F, t6556: F) -> (F, F, F, F, F, F) {
    let t16126 = F::new(4.0) / F::new(15.0) * t1972 * t5180;
    let t16130 = F::new(4.0) / F::new(15.0) * t439 * t136 * t1872 * t1968;
    let t16132 = F::new(2.0) / F::new(5.0) * t1972 * t4762;
    let t16135 = F::new(2.0) / F::new(15.0) * t439 * t6550 * t4608;
    let t16136 = F::new(4.0) / F::new(15.0) * t12075;
    let t16137 = t1423 * t6556;
    (t16126, t16130, t16132, t16135, t16136, t16137)
}
