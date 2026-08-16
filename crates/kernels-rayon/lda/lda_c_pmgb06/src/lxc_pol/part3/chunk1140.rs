//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1140/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1140(t2060: f64, t819: f64, t8088: f64, t99: f64, t2061: f64, t102: f64, t147: f64, t3092: f64, t3403: f64, t1438: f64, t472: f64, t1618: f64, t3098: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13558 = t2060 * t819;
    let t13560 = t99 * t8088;
    let t13561 = t13560 * t2061;
    let t13565 = t99 * t102 * t147;
    let t13566 = t3403 * t3092;
    let t13570 = t472 * t1438;
    let t13574 = t1618 * t3098;
    (t13558, t13560, t13561, t13565, t13566, t13570, t13574)
}
