//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 797/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk797<F: Float>(t174: F, t473: F, t903: F, t908: F, t912: F, t914: F, t1027: F, t2735: F, t2997: F, t400: F, t1059: F, t2993: F, t2998: F, t1010: F, t387: F, t1034: F, t40: F, t960: F) -> (F, F, F, F, F, F, F) {
    let t8473 = 2.291123905095794 * t174 * t473 * t903 * t908;
    let t8477 = 0.2849333333333333 * t174 * t473 * t912 * t914;
    let t8481 = 69.26302359750345 * t400 * t1027 * t2735 * t2997;
    let t8482 = t1059 * t2993;
    let t8486 = t1059 * t2998;
    let t8491 = 4.678578717964164 * t400 * t1010 * t2735 * t387;
    let t8493 = t40 * t960 * t1034;
    (t8473, t8477, t8481, t8482, t8486, t8491, t8493)
}
