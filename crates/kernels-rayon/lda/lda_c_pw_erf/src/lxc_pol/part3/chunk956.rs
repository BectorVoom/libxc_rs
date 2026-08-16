//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 956/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk956(t169: f64, t2898: f64, t632: f64, t2929: f64, t699: f64, t242: f64, t2853: f64, t299: f64, t2888: f64, t1102: f64, t1143: f64, t10810: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10893 = t169 * t2898 * t632;
    let t10897 = 0.21223777183119583_f64 * t169 * t699 * t2929;
    let t10900 = t169 * t299 * t2853 * t242;
    let t10903 = t169 * t2888 * t632;
    let t10906 = t169 * t1102 * t1143;
    let t10909 = t169 * t10810 * t242;
    (t10893, t10897, t10900, t10903, t10906, t10909)
}
