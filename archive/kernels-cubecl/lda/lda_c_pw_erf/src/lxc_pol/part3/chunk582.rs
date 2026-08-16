//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 582/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk582<F: Float>(t1191: F, t169: F, t301: F, t678: F, t1: F, t1697: F, t431: F, t119: F, t155: F, t1664: F, t411: F, t473: F) -> (F, F, F, F, F) {
    let t3203 = t169 * t1191 * t678 * t301;
    let t3210 = t431 * t1697 * t1;
    let t3212 = t119 * t155 * t1664;
    let t3213 = t3210 * t3212;
    let t3216 = t119 * t473 * t411;
    (t3203, t3210, t3212, t3213, t3216)
}
