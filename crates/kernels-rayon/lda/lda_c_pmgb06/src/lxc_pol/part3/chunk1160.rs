//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1160/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1160(t13848: f64, t5474: f64, t5499: f64, t1380: f64, t337: f64, t493: f64, t4935: f64, t497: f64, t13834: f64, t13835: f64, t13837: f64, t13839: f64, t13841: f64, t13843: f64, t13845: f64, t13847: f64) -> (f64, f64, f64, f64) {
    let t13849 = 16.0_f64 / 81.0_f64 * t13848;
    let t13850 = t5499 * t5474;
    let t13851 = 10.0_f64 / 27.0_f64 * t13850;
    let t13856 = t493 * t1380 * t4935 * t497 * t337 / 15.0_f64;
    let t13857 = t13834 - t13835 - t13837 - t13839 - t13841 + t13843 + t13845 + t13847 + t13849 + t13851 - t13856;
    (t13849, t13851, t13856, t13857)
}
