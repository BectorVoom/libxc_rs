//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1468/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1468(t6961: f64, t707: f64, t6957: f64, t10609: f64, t10614: f64, t10617: f64, t10623: f64, t10635: f64, t10640: f64, t10643: f64, t10646: f64, t10657: f64, t10661: f64, t14587: f64, t14623: f64, t14639: f64, t14642: f64, t14646: f64, t18785: f64, t18876: f64, t346: f64, t387: f64, t5583: f64, t73: f64) -> f64 {
    let t18883 = t707 * t6961;
    let t18885 = t707 * t6957;
    let t18892 = 0.008135887625008338_f64 * t10609 - t10614 + t10617 - 0.0005811348303577384_f64 * t10623 - 0.0017434044910732151_f64 * t10635 + t10640 - t10643 - 0.013430671634934398_f64 * t10646 - 0.10809180959278285_f64 * t14623 + t346 * (t18785 + t18876) * t387 * t73 + 12.0_f64 * t5583 * t14587 + 0.039914113367515366_f64 * t18883 + 0.039914113367515366_f64 * t18885 + 0.19816831758676853_f64 * t10657 - 1.849570964143173_f64 * t10661 - 0.0023245393214309535_f64 * t14639 - 0.0017434044910732151_f64 * t14642 - 0.0005811348303577384_f64 * t14646;
    t18892
}
