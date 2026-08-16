//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 555/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk555(t1030: f64, t2940: f64, t2946: f64, t400: f64, t1059: f64, t1077: f64, t659: f64, t661: f64, t1026: f64, t378: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2948 = t2946 * t2940 * t1030;
    let t2949 = t400 * t2948;
    let t2950 = 103.89453539625518_f64 * t2949;
    let t2951 = t1059 * t1077;
    let t2952 = 3.5089340384731225_f64 * t2951;
    let t2953 = 1.0_f64 / t659;
    let t2966 = 1.0_f64 / t661;
    let t2983 = 1.0_f64 / t1026 / t378;
    (t2948, t2949, t2950, t2951, t2952, t2953, t2966, t2983)
}
