//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 869/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk869<F: Float>(t174: F, t3149: F, t998: F, t155: F, t3127: F, t3131: F, t3135: F, t3137: F, t2745: F, t3123: F, t19: F, t4288: F, t729: F, t734: F) -> (F, F, F, F, F) {
    let t8389 = F::cast_from(0.07123333333333333_f64) * t174 * t998 * t3149;
    let t8393 = F::cast_from(36.84545214203136_f64) * t174 * t155 * t3127 * t3131;
    let t8397 = F::cast_from(6.873371715287382_f64) * t174 * t155 * t3135 * t3137;
    let t8400 = F::cast_from(0.4274_f64) * t174 * t2745 * t3123;
    let t8403 = t4288 * t729 * t19 * t734;
    (t8389, t8393, t8397, t8400, t8403)
}
