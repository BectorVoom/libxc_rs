//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1036/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1036<F: Float>(t108: F, t2119: F, t267: F, t3970: F, t3482: F, t3965: F, t6766: F, t2103: F, t4476: F, t9513: F, t4804: F, t4826: F) -> (F, F, F, F, F) {
    let t12136 = t2119 * t108 * t267;
    let t12138 = F::new(16.0) / F::new(15.0) * t12136 * t3970;
    let t12141 = F::new(8.0) / F::new(9.0) * t3965 * t6766 * t3482;
    let t12143 = t2103 * t108 * t267;
    let t12145 = F::new(16.0) / F::new(15.0) * t12143 * t4476;
    let t12146 = F::new(16.0) / F::new(45.0) * t9513;
    let t12148 = F::new(8.0) / F::new(15.0) * t4804 * t4826;
    (t12138, t12141, t12145, t12146, t12148)
}
