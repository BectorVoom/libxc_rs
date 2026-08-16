//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 892/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk892<F: Float>(t125: F, t2715: F, t3310: F, t8930: F, t3319: F, t8138: F, t1125: F, t427: F, t426: F, t1682: F, t474: F, t156: F, t3252: F, param_hyb_omega_0: F) -> (F, F, F, F, F, F, F) {
    let t8932 = F::cast_from(0.16322666666666666_f64) * t125 * t2715 * t3310 * t8930;
    let t8936 = F::cast_from(1.6239027777777777_f64) * param_hyb_omega_0 * t8138 * t3319 * t8930;
    let t8939 = t1125 * t427;
    let t8940 = t426 * t8939;
    let t8942 = t474 * t1682;
    let t8943 = t426 * t8942;
    let t8945 = t156 * t3252;
    (t8932, t8936, t8939, t8940, t8942, t8943, t8945)
}
