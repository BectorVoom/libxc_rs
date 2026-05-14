//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 927/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk927<F: Float>(t1427: F, t6888: F, t646: F, t7045: F, t1410: F, t2463: F, t656: F, t6881: F, t6884: F, t153: F, t474: F, t6080: F, t1210: F, t168: F, t2581: F, t635: F, t7025: F) -> (F, F, F, F, F, F, F, F) {
    let t19230 = t6888 * t1427;
    let t19249 = t7045 * t646;
    let t19256 = t2463 * t1410;
    let t19318 = t6881 * t656;
    let t19320 = t6884 * t656;
    let t19344 = t153 * t474 * t6080;
    let t19347 = t168 * t1210 * t2581;
    let t19358 = t168 * t635 * t7025;
    (t19230, t19249, t19256, t19318, t19320, t19344, t19347, t19358)
}
