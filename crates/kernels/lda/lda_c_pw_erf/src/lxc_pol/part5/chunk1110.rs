//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1110/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1110<F: Float>(t15573: F, t15587: F, t12136: F, t6759: F, t6763: F, t6767: F, t2337: F, t811: F, t3974: F, t3976: F, t593: F, t352: F) -> (F, F, F, F, F, F, F) {
    let t20688 = F::new(16.0) / F::new(45.0) * t15573;
    let t20689 = F::new(8.0) / F::new(45.0) * t15587;
    let t20691 = F::new(16.0) / F::new(15.0) * t12136 * t6759;
    let t20693 = F::new(32.0) / F::new(15.0) * t12136 * t6763;
    let t20695 = F::new(16.0) / F::new(9.0) * t12136 * t6767;
    let t20696 = t2337 * t811;
    let t20700 = F::new(8.0) / F::new(15.0) * t3974 * t3976 * t20696 * t593;
    let t20701 = t20696 * t352;
    (t20688, t20689, t20691, t20693, t20695, t20700, t20701)
}
