//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1098/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1098<F: Float>(t12841: F, t3430: F, t4763: F, t1315: F, t5327: F, t2023: F, t3727: F, t1308: F, t352: F, t5029: F, t558: F, t571: F) -> (F, F, F, F, F) {
    let t12842 = F::new(16.0) / F::new(45.0) * t12841;
    let t12844 = F::new(8.0) / F::new(9.0) * t4763 * t3430;
    let t12846 = F::new(8.0) / F::new(15.0) * t5327 * t1315;
    let t12848 = F::new(4.0) / F::new(15.0) * t3727 * t2023;
    let t12853 = F::new(4.0) / F::new(15.0) * t571 * t1308 * t5029 * t558 * t352;
    (t12842, t12844, t12846, t12848, t12853)
}
