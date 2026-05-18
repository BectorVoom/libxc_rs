//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 329/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk329<F: Float>(t1217: F, t265: F, t665: F, t668: F, t514: F, t543: F, t185: F, t473: F, t56: F, t174: F, t177: F, t325: F, t506: F) -> (F, F, F, F, F, F, F, F) {
    let t1219 = F::new(2.0) / F::new(135.0) * t265 * t1217;
    let t1220 = t665 * t668;
    let t1234 = t514 * t543;
    let t1235 = t185 * t1234;
    let t1237 = t473 * t56;
    let t1239 = t174 * t1237 * t177;
    let t1240 = F::new(0.047988888888888886) * t1239;
    let t1241 = t325 * t506;
    (t1219, t1220, t1234, t1235, t1237, t1239, t1240, t1241)
}
