//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 347/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk347<F: Float>(t1234: F, t185: F, t473: F, t56: F, t174: F, t177: F, t325: F, t506: F, t521: F) -> (F, F, F, F, F, F, F) {
    let t1235 = t185 * t1234;
    let t1236 = F::new(8.0) / F::new(45.0) * t1235;
    let t1237 = t473 * t56;
    let t1239 = t174 * t1237 * t177;
    let t1240 = F::new(0.047988888888888886) * t1239;
    let t1241 = t325 * t506;
    let t1243 = t56 * t521;
    (t1235, t1236, t1237, t1239, t1240, t1241, t1243)
}
