//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 933/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk933<F: Float>(t40: F, t60: F, t8600: F, t8639: F, t8686: F, t8729: F, t174: F, t3046: F, t3105: F, t3027: F, t3112: F, t169: F, t2817: F, t301: F, t678: F, t1063: F, t147: F) -> (F, F, F, F, F) {
    let t8733 = t40 * t60 * (t8600 + t8639 + t8686 + t8729);
    let t8734 = t60 * t174;
    let t8737 = 0.1301229705933783 * t8734 * t3046 * t3105;
    let t8740 = 1.9263778438055648 * t8734 * t3027 * t3112;
    let t8751 = t169 * t2817 * t678 * t301;
    let t8756 = t1063 * t147;
    (t8733, t8737, t8740, t8751, t8756)
}
