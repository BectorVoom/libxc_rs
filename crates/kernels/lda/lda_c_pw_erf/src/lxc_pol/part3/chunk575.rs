//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 575/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk575<F: Float>(t2954: F, t3518: F, t3516: F, t2961: F, t504: F, t538: F, t503: F, t11: F, t506: F, t925: F, t1257: F, t325: F, t1247: F, t1458: F, t56: F, t1124: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3519 = t3518 * t2954;
    let t3520 = t3516 * t3519;
    let t3523 = t504 * t2961;
    let t3524 = t538 * t3523;
    let t3527 = t503 * t3523;
    let t3528 = t11 * t3527;
    let t3530 = t925 * t506;
    let t3532 = t325 * t1257;
    let t3534 = t325 * t1247;
    let t3536 = t56 * t1458;
    let t3537 = t3536 * t3519;
    let t3538 = t11 * t3537;
    let t3540 = t1124 * t56;
    (t3520, t3524, t3527, t3528, t3530, t3532, t3534, t3536, t3537, t3538, t3540)
}
