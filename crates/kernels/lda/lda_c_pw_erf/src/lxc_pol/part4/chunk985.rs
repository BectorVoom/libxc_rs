//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 985/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk985<F: Float>(t169: F, t301: F, t5718: F, t717: F, t1549: F, t5495: F, t159: F, t285: F, t462: F, t4713: F, t4422: F, t477: F, t1128: F, t1896: F, t405: F, t5669: F) -> (F, F, F, F, F, F) {
    let t11482 = t169 * t717 * t5718 * t301;
    let t11486 = t1549 * t5495;
    let t11495 = t462 * t4713 * t159 * t285;
    let t11498 = t4422 * t477 * t285;
    let t11501 = t1896 * t1128 * t285;
    let t11507 = t405 * t5669;
    (t11482, t11486, t11495, t11498, t11501, t11507)
}
