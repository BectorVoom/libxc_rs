//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 830/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk830<F: Float>(t1244: F, t1250: F, t1275: F, t933: F, t1269: F, t331: F, t3524: F, t3520: F, t325: F, t3504: F, t3498: F, t1184: F, t56: F, t174: F, t177: F, t3490: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9777 = 1.0 / t1244 / t1250;
    let t9782 = t933 * t1275;
    let t9784 = t933 * t1269;
    let t9786 = t331 * t3524;
    let t9788 = t331 * t3520;
    let t9806 = t325 * t3504;
    let t9808 = t325 * t3498;
    let t9810 = t1184 * t56;
    let t9812 = t174 * t9810 * t177;
    let t9813 = 0.3732469135802469 * t9812;
    let t9814 = t331 * t3490;
    (t9777, t9782, t9784, t9786, t9788, t9806, t9808, t9810, t9812, t9813, t9814)
}
