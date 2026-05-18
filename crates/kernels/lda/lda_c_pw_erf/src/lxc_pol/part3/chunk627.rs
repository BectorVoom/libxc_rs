//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 627/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk627<F: Float>(t352: F, t93: F, t108: F, t2954: F, t2961: F, t2967: F, t2973: F, t3688: F, t406: F, t408: F, t659: F, t661: F, t945: F, t954: F) -> F {
    let t3695 = t93 * t352;
    let t3701 = (F::new(40.0) / F::new(27.0) * t406 * t2954 + F::new(20.0) / F::new(3.0) * t3688 * t945 + F::new(4.0) / F::new(3.0) * t659 * t2961 + F::new(40.0) / F::new(27.0) * t408 * t2967 + F::new(20.0) / F::new(3.0) * t3695 * t954 + F::new(4.0) / F::new(3.0) * t661 * t2973) * t108;
    t3701
}
