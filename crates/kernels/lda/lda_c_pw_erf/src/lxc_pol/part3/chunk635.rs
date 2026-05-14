//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 635/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk635<F: Float>(t2910: F, t482: F, t485: F, t1098: F, t1597: F, t2916: F, t1186: F, t1124: F, t465: F, t483: F, t1131: F, t1578: F, t1138: F, t2877: F, t3381: F, t3383: F, t3386: F, t3389: F, t3392: F, t3396: F, t3401: F, t3406: F, t3410: F, t3415: F, t3418: F, t3423: F, t3427: F, t3432: F, t3434: F, t3436: F) -> (F, F, F, F, F, F, F, F) {
    let t4160 = 0.005926167098672845 * t482 * t2910 * t485;
    let t4163 = 0.0014862827083471494 * t1098 * t2916 * t1597;
    let t4165 = 0.025899545097903542 * t1186 * t485;
    let t4166 = t1124 * t465;
    let t4168 = t4166 * t483 * t485;
    let t4172 = 0.01975389032890948 * t1578 * t1131 * t485;
    let t4175 = 0.0034679929861433484 * t2877 * t1138 * t1597;
    let t4176 = t3381 + t3383 + t3386 + t3389 + t3392 + t3396 + t3401 - t3406 - t3410 - t3415 + t3418 + t3423 + t3427 + t3432 - t3434 + t3436;
    (t4160, t4163, t4165, t4166, t4168, t4172, t4175, t4176)
}
