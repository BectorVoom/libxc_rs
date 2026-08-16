//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1124/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1124(t12568: f64, t1476: f64, t1830: f64, t350: f64, t4881: f64, t4886: f64, t12584: f64, t36: f64, t12594: f64, t9507: f64, t11997: f64, t506: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13343 = t1830 * t1476 * t12568;
    let t13345 = t350 * t4881;
    let t13347 = t350 * t4886;
    let t13350 = t36 * t1476 * t12584;
    let t13353 = t36 * t9507 * t12594;
    let t13356 = t1830 * t506 * t11997;
    (t13343, t13345, t13347, t13350, t13353, t13356)
}
