//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 522/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk522(t1166: f64, t1169: f64, t125: f64, t1550: f64, t1556: f64, t169: f64, t1733: f64, t1735: f64, t1808: f64, t1809: f64, t1881: f64, t1885: f64, t1891: f64, t1898: f64, t2203: f64, t2205: f64, t2208: f64, t2211: f64, t299: f64, t301: f64, t405: f64, t411: f64, t456: f64, t777: f64) -> f64 {
    let t2215 = -t777 * t1556 + t777 * t1550 + 6.0_f64 * t1808 * t1809 * t411 + t1881 * t456 - 0.054045904796391424_f64 * t1885 + 0.020267214298646783_f64 * t169 * t299 * t1891 * t301 - 0.0002905674151788692_f64 * t1898 + t2203 * t125 + 3.0_f64 * t405 * t2205 + 3.0_f64 * t1733 * t2208 + 3.0_f64 * t2211 * t1735 + 0.019957056683757683_f64 * t1166 + t1169;
    t2215
}
