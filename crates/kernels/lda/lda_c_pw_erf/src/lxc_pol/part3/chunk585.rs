//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 585/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk585<F: Float>(t261: F, t52: F, t1563: F, t352: F, t2954: F, t2961: F, t2967: F, t2973: F, t3234: F, t3237: F, t406: F, t408: F, t945: F, t954: F) -> (F, F) {
    let t3243 = F::cast_from(1.0_f64) / t52 / t261;
    let t3246 = t1563 * t352;
    let t3251 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t3234 * t2954 - t3237 * t945 / F::cast_from(3.0_f64) + t406 * t2961 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t3243 * t2967 - t3246 * t954 / F::cast_from(3.0_f64) + t408 * t2973 / F::cast_from(3.0_f64);
    (t3243, t3251)
}
