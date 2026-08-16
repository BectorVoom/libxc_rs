//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 38/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk38<F: Float>(t11: F, t14: F, t17: F, t25: F) -> (F, F, F) {
    let t80 = F::cast_from(5.1785_f64) * t14 + F::cast_from(0.905775_f64) * t11 + F::cast_from(0.1100325_f64) * t17 + F::cast_from(0.1241775_f64) * t25;
    let t83 = F::cast_from(1.0_f64) + F::cast_from(29.608574643216677_f64) / t80;
    let t84 = F::ln(t83);
    (t80, t83, t84)
}
