//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1016/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1016<F: Float>(t1298: F, t4039: F, t3604: F, t5165: F, t352: F, t743: F, t954: F) -> (F, F, F) {
    let t11906 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1298 * t4039;
    let t11907 = t5165 * t3604;
    let t11909 = t743 * t954 * t352;
    (t11906, t11907, t11909)
}
