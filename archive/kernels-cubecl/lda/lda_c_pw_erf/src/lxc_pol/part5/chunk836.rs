//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 836/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk836<F: Float>(t203: F, t7674: F, t184: F, t221: F, t2334: F, t5404: F, t1319: F, t1318: F, t6426: F, t739: F, t3806: F, t519: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7675 = t203 * t7674;
    let t7676 = t7675 * t184;
    let t7678 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t7676 * t221;
    let t7679 = t5404 * t2334;
    let t7680 = t1319 * t7679;
    let t7682 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1318 * t7680;
    let t7683 = t6426 * t739;
    let t7684 = t3806 * t7683;
    let t7686 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t519 * t7684;
    (t7675, t7676, t7678, t7679, t7680, t7682, t7683, t7684, t7686)
}
