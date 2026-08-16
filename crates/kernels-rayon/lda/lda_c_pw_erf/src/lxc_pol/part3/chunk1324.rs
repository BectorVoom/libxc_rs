//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1324/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1324(t14397: f64, t169: f64, t242: f64, t299: f64, t4713: f64, t5772: f64, t632: f64, t1143: f64, t2220: f64, t2224: f64, t2929: f64, t875: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15237 = t169 * t14397 * t242;
    let t15241 = t169 * t299 * t4713 * t242;
    let t15244 = t169 * t5772 * t632;
    let t15245 = 0.3183566577467937_f64 * t15244;
    let t15247 = t169 * t2220 * t1143;
    let t15250 = t169 * t2224 * t1143;
    let t15251 = 0.09550699732403813_f64 * t15250;
    let t15253 = t169 * t875 * t2929;
    (t15237, t15241, t15245, t15247, t15251, t15253)
}
