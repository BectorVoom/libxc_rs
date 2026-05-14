//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 944/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk944<F: Float>(t1203: F, t1590: F, t1191: F, t163: F, t169: F, t616: F, t196: F, t3674: F, t218: F, t3666: F, t1513: F, t1519: F, t3437: F, t565: F, t198: F, t4567: F) -> (F, F, F, F, F, F, F) {
    let t9207 = t1203 * t1590;
    let t9211 = t169 * t1191 * t616 * t163;
    let t9223 = 1.0 / t3674 / t196;
    let t9237 = 1.0 / t3666 / t218;
    let t9244 = t1513 * t1519;
    let t9246 = t565 * t3437;
    let t9248 = t4567 * t198;
    (t9207, t9211, t9223, t9237, t9244, t9246, t9248)
}
