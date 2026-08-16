//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 877/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk877(t134: f64, t138: f64, t9175: f64, t139: f64, t3259: f64, t1437: f64, t1830: f64, t455: f64, t1530: f64, t1710: f64, t485: f64, t1687: f64, t1730: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9177 = t138 * t9175 * t134;
    let t9178 = 0.01959135802469136_f64 * t9177;
    let t9188 = t139 * t3259;
    let t9189 = t1437 * t1437;
    let t9190 = 1.0_f64 / t9189;
    let t9215 = t1830 * t455;
    let t9220 = 1.0_f64 / t1437 / t1530;
    let t9266 = t485 * t1710;
    let t9340 = t1687 * t1730;
    (t9177, t9178, t9188, t9190, t9215, t9220, t9266, t9340)
}
