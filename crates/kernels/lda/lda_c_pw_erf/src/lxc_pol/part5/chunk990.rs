//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 990/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk990<F: Float>(t1: F, t3921: F, t5470: F, t2260: F, t3936: F, t1410: F, t2253: F, t2256: F, t3990: F, t851: F, t256: F, t3932: F, t850: F) -> (F, F, F, F, F, F) {
    let t15015 = t5470 * t1 * t3921;
    let t15060 = t2260 * t3936;
    let t15107 = t2253 * t1410;
    let t15108 = F::new(2.0) / F::new(9.0) * t15107;
    let t15109 = t2256 * t1410;
    let t15111 = t851 * t3990;
    let t15123 = t850 * t3932 * t256;
    (t15015, t15060, t15108, t15109, t15111, t15123)
}
