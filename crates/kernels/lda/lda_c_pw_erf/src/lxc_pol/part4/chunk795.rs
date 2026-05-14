//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 795/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk795<F: Float>(t169: F, t242: F, t2883: F, t2887: F, t2890: F, t2893: F, t2934: F, t5760: F, t5762: F, t5768: F, t5770: F, t5775: F, t5777: F, t1553: F, t776: F, t405: F) -> (F, F, F) {
    let t5779 = -0.28298369577492777 * t2883 - t2887 + 0.053059442957798957 * t2890 + 0.21223777183119583 * t2893 - t2934 - 0.14149184788746388 * t5760 - 0.031835665774679375 * t169 * t5762 * t242 - t5768 - 0.031835665774679375 * t5770 + t5775 + 0.10611888591559791 * t5777;
    let t5782 = t776 * t1553;
    let t5783 = t405 * t5782;
    (t5779, t5782, t5783)
}
