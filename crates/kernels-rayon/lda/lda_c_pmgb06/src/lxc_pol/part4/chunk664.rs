//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 664/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk664(t1289: f64, t377: f64, t1295: f64, t374: f64, t376: f64, t67: f64, t1180: f64, t56: f64, t69: f64, t3530: f64, t3533: f64, t3585: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3622 = t1289 * t377;
    let t3625 = t374 * t1295;
    let t3630 = t376 * t376;
    let t3631 = 1.0_f64 / t3630;
    let t3632 = t67 * t3631;
    let t3643 = 0.8940581481481481_f64 * t69 * t1180 * t56;
    let t3644 = t69 * t3530;
    let t3646 = t69 * t3533;
    let t3654 = t69 * t3585;
    (t3622, t3625, t3630, t3631, t3632, t3643, t3644, t3646, t3654)
}
