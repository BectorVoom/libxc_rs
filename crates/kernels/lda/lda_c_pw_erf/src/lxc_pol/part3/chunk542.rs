//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 542/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk542<F: Float>(t2983: F, t75: F, t2940: F, t2986: F, t2946: F, t1030: F, t2735: F, t386: F, t983: F, t991: F, t1022: F, t387: F, t385: F, t907: F, t935: F, t333: F, t904: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3081 = t75 * t2983;
    let t3082 = t2940 * t2986;
    let t3085 = t75 * t2946;
    let t3086 = t2940 * t1030;
    let t3095 = t2735 * t386;
    let t3098 = t2940 * t386;
    let t3101 = t983 * t991;
    let t3105 = t387 * t1022;
    let t3111 = t1022 * t1030;
    let t3112 = t3111 * t385;
    let t3115 = t935 * t907;
    let t3117 = t904 * t3115 * t333;
    (t3081, t3082, t3085, t3086, t3095, t3098, t3101, t3105, t3111, t3112, t3115, t3117)
}
