//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 892/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk892(t125: f64, t2715: f64, t3310: f64, t8930: f64, t3319: f64, t8138: f64, t1125: f64, t427: f64, t426: f64, t1682: f64, t474: f64, t156: f64, t3252: f64, param_hyb_omega_0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8932 = 0.16322666666666666_f64 * t125 * t2715 * t3310 * t8930;
    let t8936 = 1.6239027777777777_f64 * param_hyb_omega_0 * t8138 * t3319 * t8930;
    let t8939 = t1125 * t427;
    let t8940 = t426 * t8939;
    let t8942 = t474 * t1682;
    let t8943 = t426 * t8942;
    let t8945 = t156 * t3252;
    (t8932, t8936, t8939, t8940, t8942, t8943, t8945)
}
