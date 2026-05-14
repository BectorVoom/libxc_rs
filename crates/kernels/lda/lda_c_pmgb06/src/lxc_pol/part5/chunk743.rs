//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 743/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk743<F: Float>(t3248: F, t7566: F, t493: F, t2002: F, t2481: F, t2485: F, t1962: F, t2480: F, t439: F, t2484: F, t4619: F, t444: F, t7290: F, t442: F, t3261: F, t7284: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7567 = t3248 * t7566;
    let t7569 = 8.0 / 81.0 * t493 * t7567;
    let t7571 = t2002 * t2481 / 15.0;
    let t7573 = t2002 * t2485 / 9.0;
    let t7574 = t1962 * t2480;
    let t7576 = t439 * t7574 / 15.0;
    let t7577 = t4619 * t2484;
    let t7579 = t439 * t7577 / 9.0;
    let t7580 = t444 * t7290;
    let t7581 = t442 * t7580;
    let t7583 = t439 * t7581 / 45.0;
    let t7584 = t3261 * t7284;
    (t7567, t7569, t7571, t7573, t7574, t7576, t7577, t7579, t7580, t7581, t7583, t7584)
}
