//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 806/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk806<F: Float>(t439: F, t7574: F, t2484: F, t4619: F, t444: F, t7290: F, t442: F, t3261: F, t7284: F, t3260: F, t2555: F, t831: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7576 = t439 * t7574 / F::new(15.0);
    let t7577 = t4619 * t2484;
    let t7579 = t439 * t7577 / F::new(9.0);
    let t7580 = t444 * t7290;
    let t7581 = t442 * t7580;
    let t7583 = t439 * t7581 / F::new(45.0);
    let t7584 = t3261 * t7284;
    let t7585 = t3260 * t7584;
    let t7587 = F::new(8.0) / F::new(81.0) * t439 * t7585;
    let t7589 = t831 * t2555 / F::new(10.0);
    (t7576, t7577, t7579, t7580, t7581, t7583, t7584, t7585, t7587, t7589)
}
