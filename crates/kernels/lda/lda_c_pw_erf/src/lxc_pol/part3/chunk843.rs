//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 843/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk843<F: Float>(t1446: F, t3880: F, t3884: F, t3788: F, t3794: F, t1449: F, t3738: F, t519: F, t1335: F, t3762: F, t571: F, t1318: F, t3420: F, t3854: F, t4049: F, t581: F) -> (F, F, F, F, F, F, F) {
    let t10320 = t1446 * t3880;
    let t10322 = t1446 * t3884;
    let t10326 = t3794 * t3788;
    let t10350 = t519 * t1449 * t3738;
    let t10361 = t571 * t3762 * t1335;
    let t10371 = t1318 * t3854 * t3420;
    let t10379 = t4049 * t581;
    (t10320, t10322, t10326, t10350, t10361, t10371, t10379)
}
