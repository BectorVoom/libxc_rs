//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 744/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk744<F: Float>(t3260: F, t7584: F, t439: F, t2555: F, t831: F, t2911: F, t7295: F, t2909: F, t36: F, t2918: F, t1476: F, t7516: F, t1464: F, t506: F, t7512: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7585 = t3260 * t7584;
    let t7587 = 8.0 / 81.0 * t439 * t7585;
    let t7589 = t831 * t2555 / 10.0;
    let t7594 = t2911 * t7295;
    let t7595 = t2909 * t7594;
    let t7596 = t36 * t7595;
    let t7598 = t2918 * t7295;
    let t7599 = t1476 * t7598;
    let t7600 = t36 * t7599;
    let t7602 = t1476 * t7516;
    let t7603 = t36 * t7602;
    let t7605 = t1464 * t7295;
    let t7606 = t506 * t7605;
    let t7607 = t36 * t7606;
    let t7609 = t506 * t7512;
    let t7610 = t36 * t7609;
    (t7585, t7587, t7589, t7594, t7595, t7596, t7598, t7599, t7600, t7602, t7603, t7605, t7606, t7607, t7609, t7610)
}
