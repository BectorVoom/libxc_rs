//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 827/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk827<F: Float>(t142: F, t2594: F, t455: F, t2610: F, t2325: F, t3234: F, t1558: F, t2329: F, t2334: F, t3243: F, t1563: F, t2337: F, t1820: F, t1826: F, t348: F, t352: F, t406: F, t408: F, t5992: F, t6005: F, t943: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6093 = t142 * t2594;
    let t6094 = t455 * t6093;
    let t6097 = t142 * t2610;
    let t6098 = t455 * t6097;
    let t6101 = t3234 * t2325;
    let t6106 = t1558 * t2329;
    let t6111 = t3243 * t2334;
    let t6116 = t1563 * t2337;
    let t6121 = 4.0 / 27.0 * t6101 * t348 - 4.0 / 9.0 * t1820 * t943 - t6106 * t348 / 9.0 + t406 * t5992 / 3.0 + 4.0 / 27.0 * t6111 * t352 + 4.0 / 9.0 * t1826 * t943 - t6116 * t352 / 9.0 + t408 * t6005 / 3.0;
    (t6093, t6094, t6097, t6098, t6101, t6106, t6111, t6116, t6121)
}
