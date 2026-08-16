//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1094/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1094(t154: f64, t3092: f64, t12398: f64, t13026: f64, t465: f64, t13002: f64, t1395: f64, t1438: f64, t5083: f64, t5086: f64, t12987: f64, t5084: f64) -> (f64, f64, f64, f64) {
    let t13027 = t154 * t3092;
    let t13030 = 8.0_f64 / 27.0_f64 * t13026 * t13027 * t12398;
    let t13031 = t465 * t3092;
    let t13034 = 8.0_f64 / 27.0_f64 * t13026 * t13031 * t13002;
    let t13035 = t1395 * t1438;
    let t13038 = 2.0_f64 / 9.0_f64 * t5083 * t13035 * t5086;
    let t13041 = t5083 * t5084 * t12987 / 9.0_f64;
    (t13030, t13034, t13038, t13041)
}
