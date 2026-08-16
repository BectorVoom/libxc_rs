//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 806/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk806(t439: f64, t7574: f64, t2484: f64, t4619: f64, t444: f64, t7290: f64, t442: f64, t3261: f64, t7284: f64, t3260: f64, t2555: f64, t831: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7576 = t439 * t7574 / 15.0_f64;
    let t7577 = t4619 * t2484;
    let t7579 = t439 * t7577 / 9.0_f64;
    let t7580 = t444 * t7290;
    let t7581 = t442 * t7580;
    let t7583 = t439 * t7581 / 45.0_f64;
    let t7584 = t3261 * t7284;
    let t7585 = t3260 * t7584;
    let t7587 = 8.0_f64 / 81.0_f64 * t439 * t7585;
    let t7589 = t831 * t2555 / 10.0_f64;
    (t7576, t7577, t7579, t7580, t7581, t7583, t7584, t7585, t7587, t7589)
}
