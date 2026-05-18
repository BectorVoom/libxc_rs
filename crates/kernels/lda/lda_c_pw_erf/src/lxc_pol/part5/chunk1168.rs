//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1168/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1168<F: Float>(t21348: F, t15563: F, t799: F, t2178: F, t6988: F, t15694: F, t2558: F, t3883: F, t519: F, t7484: F, t2171: F, t6682: F) -> (F, F, F, F, F, F) {
    let t21349 = F::new(16.0) / F::new(45.0) * t21348;
    let t21351 = F::new(8.0) / F::new(15.0) * t15563 * t799;
    let t21353 = F::new(16.0) / F::new(15.0) * t6988 * t2178;
    let t21355 = F::new(8.0) / F::new(5.0) * t15694 * t2558;
    let t21357 = t519 * t3883 * t7484;
    let t21358 = F::new(16.0) / F::new(27.0) * t21357;
    let t21359 = t2171 * t6682;
    (t21349, t21351, t21353, t21355, t21358, t21359)
}
