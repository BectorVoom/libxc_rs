//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 658/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk658(t170: f64, t3457: f64, t117: f64, t123: f64, t550: f64, t740: f64, t1650: f64, t315: f64, t1135: f64, t118: f64, t103: f64, t37: f64) -> (f64, f64, f64, f64, f64) {
    let t3458 = t170 * t3457;
    let t3474 = t123 * t740 * t550 * t117;
    let t3478 = t123 * t315 * t1650 * t117;
    let t3481 = 0.1890324433388467_f64 * t1135 * t118;
    let t3500 = 1.0_f64 / t37 / t103 / 4.0_f64;
    (t3458, t3474, t3478, t3481, t3500)
}
