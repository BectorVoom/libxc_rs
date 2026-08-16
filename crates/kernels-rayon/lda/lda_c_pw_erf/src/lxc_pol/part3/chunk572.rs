//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 572/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk572(t2983: f64, t75: f64, t2940: f64, t2986: f64, t2946: f64, t1030: f64, t2735: f64, t386: f64, t983: f64, t991: f64, t1022: f64, t387: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3081 = t75 * t2983;
    let t3082 = t2940 * t2986;
    let t3085 = t75 * t2946;
    let t3086 = t2940 * t1030;
    let t3095 = t2735 * t386;
    let t3098 = t2940 * t386;
    let t3101 = t983 * t991;
    let t3105 = t387 * t1022;
    (t3081, t3082, t3085, t3086, t3095, t3098, t3101, t3105)
}
