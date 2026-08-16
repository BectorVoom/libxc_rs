//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 807/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk807(t2911: f64, t7295: f64, t2909: f64, t36: f64, t2918: f64, t1476: f64, t7516: f64, t1464: f64, t506: f64, t7512: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7594 = t2911 * t7295;
    let t7595 = t2909 * t7594;
    let t7596 = t36 * t7595;
    let t7598 = t2918 * t7295;
    let t7599 = t1476 * t7598;
    let t7600 = t36 * t7599;
    let t7602 = t1476 * t7516;
    let t7603 = t36 * t7602;
    let t7605 = t1464 * t7295;
    let t7606 = t506 * t7605;
    let t7607 = t36 * t7606;
    let t7609 = t506 * t7512;
    (t7594, t7595, t7596, t7598, t7599, t7600, t7602, t7603, t7605, t7606, t7607, t7609)
}
