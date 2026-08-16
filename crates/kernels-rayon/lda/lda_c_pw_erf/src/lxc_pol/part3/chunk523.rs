//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 523/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk523(t299: f64, t780: f64, t169: f64, t242: f64, t171: f64, t1904: f64, t632: f64, t875: f64, t462: f64, t865: f64, t1101: f64, t1104: f64, t1108: f64, t1118: f64, t1146: f64, t1148: f64, t1149: f64, t145: f64, t1891: f64) -> (f64, f64, f64) {
    let t2220 = t299 * t780;
    let t2222 = t169 * t2220 * t242;
    let t2224 = t171 * t1904;
    let t2229 = t169 * t875 * t632;
    let t2233 = t462 * t865;
    let t2237 = -t1101 + 0.053059442957798957_f64 * t1104 + t1108 + 0.053059442957798957_f64 * t2222 - 0.031835665774679375_f64 * t169 * t2224 * t242 - 0.031835665774679375_f64 * t2229 - 0.031835665774679375_f64 * t1118 - t1146 + t1148 - 0.10665013548435875_f64 * t1149 - 0.10665013548435875_f64 * t2233 + 0.05332506774217938_f64 * t145 * t1891;
    (t2220, t2224, t2237)
}
