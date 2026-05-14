//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 640/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk640<F: Float>(t1335: F, t1475: F, t571: F, t1472: F, t1476: F, t155: F, t573: F) -> (F, F, F, F) {
    let t3748 = t1475 * t1335;
    let t3749 = t571 * t3748;
    let t3760 = t1472 * t1476;
    let t3762 = t155 * t573;
    (t3748, t3749, t3760, t3762)
}
