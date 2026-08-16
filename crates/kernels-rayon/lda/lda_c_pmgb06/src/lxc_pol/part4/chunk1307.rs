//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1307/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1307(t1476: f64, t16354: f64, t36: f64, t350: f64, t6813: f64, t405: f64, t6882: f64, t1080: f64, t2389: f64, t2918: f64, t15200: f64, t506: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17175 = t36 * t1476 * t16354;
    let t17177 = t350 * t6813;
    let t17185 = t405 * t6882;
    let t17188 = t2918 * t2389 * t1080;
    let t17190 = t36 * t1476 * t17188;
    let t17193 = t36 * t506 * t15200;
    (t17175, t17177, t17185, t17188, t17190, t17193)
}
