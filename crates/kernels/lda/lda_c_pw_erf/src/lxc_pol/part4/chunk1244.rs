//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1244/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1244<F: Float>(t14107: F, t1446: F, t6699: F, t1472: F, t6702: F, t1475: F, t571: F, t6924: F, t6705: F, t2188: F, t5327: F, t2178: F, t2550: F, t3709: F, t6939: F, t2554: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18484 = 16.0 / 45.0 * t14107;
    let t18485 = t1446 * t6699;
    let t18486 = 16.0 / 81.0 * t18485;
    let t18487 = t1472 * t6702;
    let t18488 = 16.0 / 135.0 * t18487;
    let t18490 = t571 * t1475 * t6924;
    let t18491 = 16.0 / 135.0 * t18490;
    let t18492 = t1472 * t6705;
    let t18493 = 16.0 / 81.0 * t18492;
    let t18495 = 16.0 / 15.0 * t5327 * t2188;
    let t18497 = 32.0 / 45.0 * t5327 * t2178;
    let t18499 = 4.0 / 45.0 * t3709 * t2550;
    let t18501 = 8.0 / 45.0 * t1446 * t6939;
    let t18503 = 4.0 / 27.0 * t3709 * t2554;
    (t18484, t18486, t18488, t18491, t18493, t18495, t18497, t18499, t18501, t18503)
}
