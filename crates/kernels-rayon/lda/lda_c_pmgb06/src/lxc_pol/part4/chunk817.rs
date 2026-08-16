//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 817/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk817(t247: f64, t794: f64, t113: f64, t301: f64, t1147: f64, t123: f64, t317: f64, t902: f64, t1798: f64, t395: f64, t2174: f64, t413: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5567 = t247 * t794;
    let t5569 = t5567 * t113 * t301;
    let t5573 = t123 * t1147 * t902 * t317;
    let t5575 = t395 * t1798;
    let t5578 = 0.0005811348303577384_f64 * t5575 * t113 * t301;
    let t5580 = t2174 * t413 * t301;
    (t5567, t5569, t5573, t5575, t5578, t5580)
}
