//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 915/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk915<F: Float>(t10030: F, t6725: F, t1519: F, t2443: F, t2137: F, t6851: F, t519: F, t5237: F, t6352: F, t3863: F, t571: F, t6356: F, t3854: F, t6361: F, t4794: F, t6366: F) -> (F, F, F, F, F, F, F) {
    let t18192 = t10030 * t6725;
    let t18280 = t2443 * t1519;
    let t18292 = t6851 * t2137;
    let t18308 = t519 * t5237 * t6352;
    let t18311 = t571 * t3863 * t6356;
    let t18314 = t571 * t3854 * t6361;
    let t18317 = t571 * t4794 * t6366;
    (t18192, t18280, t18292, t18308, t18311, t18314, t18317)
}
