//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 609/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk609(t123: f64, t199: f64, t4429: f64, t395: f64, t2799: f64, t1156: f64, t868: f64, t1808: f64, t722: f64, t1798: f64, t315: f64, t2281: f64, t566: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4431 = t123 * t4429 * t199;
    let t4433 = 2.0_f64 * t395;
    let t4434 = 6.0_f64 * t2799;
    let t4441 = 0.10611888591559791_f64 * t123 * t1156 * t868;
    let t4444 = 0.10611888591559791_f64 * t123 * t722 * t1808;
    let t4454 = t315 * t1798;
    let t4457 = 0.10611888591559791_f64 * t123 * t4454 * t199;
    let t4460 = 0.10611888591559791_f64 * t123 * t2281 * t566;
    (t4431, t4433, t4434, t4441, t4444, t4454, t4457, t4460)
}
