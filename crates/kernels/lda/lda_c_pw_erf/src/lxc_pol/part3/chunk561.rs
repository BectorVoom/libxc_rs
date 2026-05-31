//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 561/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk561<F: Float>(t43: F, t2953: F, t2954: F, t2957: F, t2961: F, t47: F, t945: F, t661: F, t352: F, t951: F, zeta_threshold: F) -> (F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t2965 = piecewise3::<F>(t44, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t2953 * t2954 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2957 * t945 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t47 * t2961);
    let t2966 = F::cast_from(1.0_f64) / t661;
    let t2967 = t951 * t352;
    (t2965, t2966, t2967)
}
