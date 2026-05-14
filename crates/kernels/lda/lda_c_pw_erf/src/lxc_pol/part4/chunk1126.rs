//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1126/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1126<F: Float>(t611: F, t7280: F, t9434: F, t9437: F, t2120: F, t4039: F, t12119: F, t12129: F, t1472: F, t6685: F, t2329: F, t494: F, t348: F, t3965: F, t5147: F, t542: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t16529 = t7280 * t611;
    let t16531 = 16.0 / 135.0 * t9434;
    let t16532 = 64.0 / 1215.0 * t9437;
    let t16534 = 8.0 / 15.0 * t2120 * t4039;
    let t16535 = 32.0 / 81.0 * t12119;
    let t16536 = 64.0 / 135.0 * t12129;
    let t16537 = t1472 * t6685;
    let t16538 = 32.0 / 135.0 * t16537;
    let t16539 = t2329 * t494;
    let t16540 = t16539 * t348;
    let t16543 = 16.0 / 27.0 * t3965 * t5147 * t16540;
    let t16545 = t2329 * t542 * t348;
    (t16529, t16531, t16532, t16534, t16535, t16536, t16538, t16539, t16540, t16543, t16545)
}
