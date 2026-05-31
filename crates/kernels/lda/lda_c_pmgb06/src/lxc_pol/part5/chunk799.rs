//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 799/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk799<F: Float>(t1525: F, t7485: F, t36: F, t1438: F, t7284: F, t453: F, t1863: F, t2381: F, t443: F, t7290: F, t3081: F, t4635: F, t6205: F, t6207: F, t6209: F, t7479: F, t7483: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
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
    let t7501 = t3081 + F::cast_from(0.002518888888888889_f64) * t4635 - F::cast_from(0.0012594444444444445_f64) * t6205 + F::cast_from(0.003778333333333333_f64) * t6207 - F::cast_from(0.0018891666666666666_f64) * t6209 + F::cast_from(0.002099074074074074_f64) * t7479 - F::cast_from(0.007556666666666666_f64) * t7483 + F::cast_from(0.003778333333333333_f64) * t7487 + F::cast_from(0.011335_f64) * t7491 - F::cast_from(0.011335_f64) * t7495 + F::cast_from(0.0018891666666666666_f64) * t7499;
    (t7486, t7487, t7489, t7490, t7491, t7493, t7494, t7495, t7497, t7498, t7499, t7501)
}
