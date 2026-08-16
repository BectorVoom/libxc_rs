//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1232/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1232(t342: f64, t4232: f64, t4354: f64, t301: f64, t413: f64, t5575: f64, t1183: f64, t2174: f64, t113: f64, t395: f64, t4463: f64, t4394: f64, t73: f64) -> (f64, f64, f64, f64, f64) {
    let t14633 = t4232 * t4354 * t342;
    let t14639 = t5575 * t413 * t301;
    let t14640 = 0.0017434044910732151_f64 * t14639;
    let t14642 = t2174 * t1183 * t301;
    let t14646 = t395 * t4463 * t113 * t301;
    let t14648 = t73 * t4394;
    (t14633, t14640, t14642, t14646, t14648)
}
