//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1026/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1026<F: Float>(t2076: F, t3390: F, t4729: F, t511: F, t2061: F, t830: F, t11845: F, t2062: F, t190: F, t4981: F, t9821: F, t325: F, t4681: F, t4667: F, t4606: F, t4677: F) -> (F, F, F, F, F, F, F, F) {
    let t13548 = t2076 * t3390;
    let t13550 = t511 * t4729;
    let t13562 = t2061 * t830;
    let t13564 = t11845 * t2062;
    let t13583 = t190 * t9821 * t4981;
    let t13585 = t325 * t4681;
    let t13587 = t325 * t4667;
    let t13589 = t4606 * t4677;
    (t13548, t13550, t13562, t13564, t13583, t13585, t13587, t13589)
}
