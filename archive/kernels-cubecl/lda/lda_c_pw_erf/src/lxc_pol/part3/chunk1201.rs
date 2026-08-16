//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1201/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1201<F: Float>(t14131: F, t14152: F, t184: F, t203: F, t221: F, t2954: F, t3518: F, t519: F, t806: F, t9700: F, t4753: F, t5226: F) -> (F, F, F) {
    let t14157 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t203 * (t14131 + t14152) * t184 * t221;
    let t14162 = F::cast_from(32.0_f64) / F::cast_from(81.0_f64) * t519 * t9700 * t806 * t3518 * t2954;
    let t14164 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4753 * t5226;
    (t14157, t14162, t14164)
}
