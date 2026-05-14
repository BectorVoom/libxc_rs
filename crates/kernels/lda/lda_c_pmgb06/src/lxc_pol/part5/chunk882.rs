//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 882/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk882<F: Float>(t1392: F, t2592: F, t2466: F, t3226: F, t1447: F, t6541: F, t6545: F, t2470: F, t6282: F, t1969: F, t5220: F, t6287: F, t6528: F, t2614: F, t955: F, t2617: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t16936 = t2592 * t1392;
    let t16962 = t3226 * t2466;
    let t16964 = t1447 * t6541;
    let t16966 = t1447 * t6545;
    let t16968 = t3226 * t2470;
    let t16970 = t1447 * t6282;
    let t16992 = t5220 * t1969;
    let t17004 = t1447 * t6287;
    let t17006 = t1447 * t6528;
    let t17025 = t955 * t2614;
    let t17030 = t955 * t2617;
    (t16936, t16962, t16964, t16966, t16968, t16970, t16992, t17004, t17006, t17025, t17030)
}
