//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1117/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1117(t1420: f64, t4620: f64, t10082: f64, t10083: f64, t10085: f64, t13257: f64, t13258: f64, t13260: f64, t13262: f64, t13264: f64, t13266: f64, t13268: f64, t13270: f64) -> (f64, f64) {
    let t13272 = t1420 * t4620 / 9.0_f64;
    let t13273 = t10082 - 2.0_f64 / 45.0_f64 * t10083 + 2.0_f64 / 45.0_f64 * t10085 - t13257 - t13258 + t13260 + t13262 + t13264 + t13266 + t13268 + t13270 + t13272;
    (t13272, t13273)
}
