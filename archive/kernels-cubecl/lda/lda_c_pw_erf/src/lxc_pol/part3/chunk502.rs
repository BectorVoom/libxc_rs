//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 502/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk502<F: Float>(t2072: F, t548: F, t202: F, t820: F, t184: F) -> (F, F, F) {
    let t2074 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t548 * t2072;
    let t2075 = t202 * t820;
    let t2076 = t2075 * t184;
    (t2074, t2075, t2076)
}
