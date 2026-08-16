//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1125/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1125(t12588: f64, t1476: f64, t1830: f64, t12599: f64, t2909: f64, t12605: f64, t36: f64, t506: f64, t12558: f64, t1827: f64, t947: f64, t1822: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13359 = t1830 * t1476 * t12588;
    let t13362 = t1830 * t2909 * t12599;
    let t13365 = t36 * t506 * t12605;
    let t13368 = t1830 * t506 * t12558;
    let t13370 = t947 * t1827;
    let t13372 = t947 * t1822;
    (t13359, t13362, t13365, t13368, t13370, t13372)
}
