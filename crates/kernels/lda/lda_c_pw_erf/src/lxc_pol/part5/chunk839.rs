//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 839/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk839<F: Float>(t1159: F, t646: F, t1426: F, t695: F, t3926: F, t458: F, t1155: F, t10682: F, t3921: F, t3949: F, t656: F, t1423: F, t3915: F, t3933: F, t1: F, t4166: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11022 = t1159 * t646;
    let t11025 = 0.26596355555555556 * t695 * t1426;
    let t11027 = 0.19947266666666666 * t458 * t3926;
    let t11029 = 0.19208479012345678 * t1155 * t646;
    let t11038 = 0.008082336938271605 * t10682 * t3921;
    let t11057 = t3949 * t656;
    let t11060 = t1423 * t3915;
    let t11063 = 8.0 / 9.0 * t3933 * t656;
    let t11065 = t4166 * t1 * t3921;
    (t11022, t11025, t11027, t11029, t11038, t11057, t11060, t11063, t11065)
}
