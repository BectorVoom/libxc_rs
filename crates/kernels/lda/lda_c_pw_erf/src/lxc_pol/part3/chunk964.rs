//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 964/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk964<F: Float>(t19: F, t2877: F, t646: F, t732: F, t1423: F, t3927: F, t24: F, t247: F, t3932: F, t645: F, t256: F, t639: F) -> (F, F, F, F) {
    let t11073 = F::new(0.0002763148940771605) * t2877 * t19 * t732 * t646;
    let t11074 = t1423 * t3927;
    let t11079 = F::new(0.2431111111111111) * t645 * t24 * t247 * t3932;
    let t11081 = t639 * t3932 * t256;
    (t11073, t11074, t11079, t11081)
}
