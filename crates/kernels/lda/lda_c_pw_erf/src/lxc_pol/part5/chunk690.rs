//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 690/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk690<F: Float>(t100: F, t411: F, t142: F, t1859: F, t1554: F, t169: F, t2357: F, t301: F, t717: F, t2363: F, t462: F, t159: F, t285: F) -> (F, F, F, F, F, F) {
    let t6126 = t411 * t100;
    let t6129 = t142 * t1859;
    let t6130 = t1554 * t6129;
    let t6136 = t169 * t717 * t2357 * t301;
    let t6138 = t462 * t2363;
    let t6140 = t6138 * t159 * t285;
    (t6126, t6129, t6130, t6136, t6138, t6140)
}
