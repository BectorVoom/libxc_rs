//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1131/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1131(t123: f64, t199: f64, t315: f64, t4463: f64, t4454: f64, t566: f64, t4259: f64, t868: f64, t1156: f64, t1808: f64, t2771: f64, t4351: f64) -> (f64, f64, f64, f64, f64) {
    let t14723 = t123 * t315 * t4463 * t199;
    let t14726 = t123 * t4454 * t566;
    let t14741 = t123 * t4259 * t868;
    let t14744 = t123 * t1156 * t1808;
    let t14758 = t4351 * t2771;
    (t14723, t14726, t14741, t14744, t14758)
}
