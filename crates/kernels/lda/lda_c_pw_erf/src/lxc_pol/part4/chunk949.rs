//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 949/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk949<F: Float>(t1184: F, t56: F, t174: F, t177: F, t1191: F, t191: F, t187: F, t190: F, t1257: F, t925: F, t1247: F, t3892: F, t1953: F, t506: F, t1253: F, t1272: F, t933: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9810 = t1184 * t56;
    let t9812 = t174 * t9810 * t177;
    let t9813 = 0.3732469135802469 * t9812;
    let t9821 = t1191 * t191;
    let t9824 = 0.10864197530864197 * t190 * t9821 * t187;
    let t9828 = t925 * t1257;
    let t9832 = t925 * t1247;
    let t9836 = t56 * t3892;
    let t9847 = t1953 * t506;
    let t9866 = t925 * t1253;
    let t9891 = t933 * t1272;
    (t9810, t9812, t9813, t9821, t9824, t9828, t9832, t9836, t9847, t9866, t9891)
}
