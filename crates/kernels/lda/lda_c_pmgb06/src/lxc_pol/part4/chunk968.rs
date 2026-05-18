//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 968/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk968<F: Float>(t1186: F, t1770: F, t4243: F, t1768: F, t2837: F, t398: F, t419: F, t4238: F, t4239: F, t1099: F, t33: F, t83: F) -> (F, F, F, F, F, F) {
    let t8070 = t4243 * t1186 * t1770;
    let t8074 = F::new(0.00010931146159029059) * t1768 * t2837 * t1770;
    let t8077 = t4238 * t398 * t419 * t1770;
    let t8081 = F::new(0.0006558687695417436) * t4239 * t1186 * t1770;
    let t8083 = F::new(1.0) / t33 / t1099;
    let t8085 = t8083 * t83 * t419;
    (t8070, t8074, t8077, t8081, t8083, t8085)
}
