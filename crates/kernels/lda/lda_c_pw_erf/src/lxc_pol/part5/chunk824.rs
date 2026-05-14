//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 824/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk824<F: Float>(t1244: F, t2061: F, t539: F, t1250: F, t1184: F, t56: F, t174: F, t177: F, t1191: F, t191: F, t187: F, t190: F, t3892: F, t1953: F, t506: F, t2070: F, t594: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9762 = t1244 * t1244;
    let t9763 = 1.0 / t9762;
    let t9772 = t2061 * t539;
    let t9777 = 1.0 / t1244 / t1250;
    let t9810 = t1184 * t56;
    let t9812 = t174 * t9810 * t177;
    let t9813 = 0.3732469135802469 * t9812;
    let t9821 = t1191 * t191;
    let t9824 = 0.10864197530864197 * t190 * t9821 * t187;
    let t9836 = t56 * t3892;
    let t9847 = t1953 * t506;
    let t9933 = t2070 * t594;
    (t9763, t9772, t9777, t9810, t9812, t9813, t9821, t9824, t9836, t9847, t9933)
}
