//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 827/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk827<F: Float>(t1904: F, t299: F, t169: F, t242: F, t2220: F, t632: F, t2883: F, t2887: F, t2890: F, t2893: F, t2934: F, t5760: F, t5762: F, t5768: F, t5770: F) -> (F, F) {
    let t5772 = t299 * t1904;
    let t5775 = F::cast_from(0.10611888591559791_f64) * t169 * t5772 * t242;
    let t5777 = t169 * t2220 * t632;
    let t5779 = -F::cast_from(0.28298369577492777_f64) * t2883 - t2887 + F::cast_from(0.053059442957798957_f64) * t2890 + F::cast_from(0.21223777183119583_f64) * t2893 - t2934 - F::cast_from(0.14149184788746388_f64) * t5760 - F::cast_from(0.031835665774679375_f64) * t169 * t5762 * t242 - t5768 - F::cast_from(0.031835665774679375_f64) * t5770 + t5775 + F::cast_from(0.10611888591559791_f64) * t5777;
    (t5772, t5779)
}
