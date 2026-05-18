//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 903/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk903<F: Float>(t145: F, t2853: F, t164: F, t4100: F, t479: F, t4120: F, t1198: F, t1590: F, t4263: F, t458: F, t1203: F, t1191: F, t163: F, t169: F, t616: F) -> (F, F, F, F, F, F, F, F) {
    let t9196 = t145 * t2853;
    let t9197 = t9196 * t164;
    let t9199 = t4100 * t479;
    let t9201 = t4120 * t164;
    let t9203 = t1198 * t1590;
    let t9206 = F::new(0.12602162889256446) * t458 * t4263;
    let t9207 = t1203 * t1590;
    let t9211 = t169 * t1191 * t616 * t163;
    (t9196, t9197, t9199, t9201, t9203, t9206, t9207, t9211)
}
