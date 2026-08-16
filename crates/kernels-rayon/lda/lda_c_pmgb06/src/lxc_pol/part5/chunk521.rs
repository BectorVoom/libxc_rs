//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 521/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk521(t166: f64, t2653: f64, t161: f64, t1732: f64, t2586: f64, t2594: f64, t2596: f64, t2598: f64, t2603: f64, t2608: f64, t2627: f64, t2629: f64, t2633: f64, t2652: f64) -> (f64, f64, f64) {
    let t2654 = t166 * t2653;
    let t2656 = t161 * t2654 / 15.0_f64;
    let t2657 = t2586 + t2594 + t2596 - t2598 + t2603 + t2608 - t2627 - t2629 - t2633 - t2652 - t2656 + t1732;
    (t2654, t2656, t2657)
}
