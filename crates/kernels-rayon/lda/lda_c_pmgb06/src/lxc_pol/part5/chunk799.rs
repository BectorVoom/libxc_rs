//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 799/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk799(t1525: f64, t7485: f64, t36: f64, t1438: f64, t7284: f64, t453: f64, t1863: f64, t2381: f64, t443: f64, t7290: f64, t3081: f64, t4635: f64, t6205: f64, t6207: f64, t6209: f64, t7479: f64, t7483: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7486 = t1525 * t7485;
    let t7487 = t36 * t7486;
    let t7489 = t1438 * t7284;
    let t7490 = t453 * t7489;
    let t7491 = t36 * t7490;
    let t7493 = t1863 * t2381;
    let t7494 = t453 * t7493;
    let t7495 = t36 * t7494;
    let t7497 = t443 * t7290;
    let t7498 = t453 * t7497;
    let t7499 = t36 * t7498;
    let t7501 = t3081 + 0.002518888888888889_f64 * t4635 - 0.0012594444444444445_f64 * t6205 + 0.003778333333333333_f64 * t6207 - 0.0018891666666666666_f64 * t6209 + 0.002099074074074074_f64 * t7479 - 0.007556666666666666_f64 * t7483 + 0.003778333333333333_f64 * t7487 + 0.011335_f64 * t7491 - 0.011335_f64 * t7495 + 0.0018891666666666666_f64 * t7499;
    (t7486, t7487, t7489, t7490, t7491, t7493, t7494, t7495, t7497, t7498, t7499, t7501)
}
