//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1031/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1031(t10547: f64, t1322: f64, t384: f64, t123: f64, t290: f64, t317: f64, t8101: f64, t113: f64, t301: f64, t3951: f64, t83: f64, t1166: f64, t247: f64) -> (f64, f64, f64, f64, f64) {
    let t10548 = 1.0_f64 / t10547;
    let t10577 = t1322 * t384;
    let t10599 = 5.240451065072324_f64 * t123 * t8101 * t290 * t317;
    let t10603 = 1.0943113336969376e-06_f64 * t3951 * t83 * t113 * t301;
    let t10606 = t247 * t1166 * t113 * t301;
    (t10548, t10577, t10599, t10603, t10606)
}
