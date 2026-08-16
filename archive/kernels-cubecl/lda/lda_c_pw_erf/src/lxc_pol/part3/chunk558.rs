//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 558/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk558<F: Float>(t1027: F, t2940: F, t386: F, t400: F, t1026: F, t80: F) -> (F, F, F, F) {
    let t2942 = t1027 * t2940 * t386;
    let t2943 = t400 * t2942;
    let t2944 = F::cast_from(3.5089340384731225_f64) * t2943;
    let t2946 = F::cast_from(1.0_f64) / t1026 / t80;
    (t2942, t2943, t2944, t2946)
}
