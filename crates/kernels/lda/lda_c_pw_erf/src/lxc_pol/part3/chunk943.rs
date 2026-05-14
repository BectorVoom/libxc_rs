//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 943/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk943<F: Float>(t3704: F, t3964: F, t34: F, t494: F, t3967: F, t542: F, t10015: F, t5143: F, t12446: F, t3965: F, t5141: F, t12450: F, t12025: F, t12389: F, t348: F, t4576: F, t565: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12475 = t3964 * t3704;
    let t12476 = t34 * t494;
    let t12480 = 32.0 / 15.0 * t12475 * t3967 * t12476 * t542;
    let t12482 = 32.0 / 15.0 * t10015 * t5143;
    let t12485 = 16.0 / 15.0 * t3965 * t5141 * t12446;
    let t12488 = 16.0 / 15.0 * t3965 * t5141 * t12450;
    let t12491 = 16.0 / 3.0 * t3965 * t12025 * t12389;
    let t12492 = t12476 * t348;
    let t12495 = 64.0 / 15.0 * t12475 * t5141 * t12492;
    let t12497 = 8.0 / 15.0 * t565 * t4576;
    (t12475, t12480, t12482, t12485, t12488, t12491, t12492, t12495, t12497)
}
