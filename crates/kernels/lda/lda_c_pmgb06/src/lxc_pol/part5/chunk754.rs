//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 754/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk754<F: Float>(t2093: F, t2623: F, t166: F, t161: F, t2625: F, t831: F, t2592: F, t824: F, t2631: F, t802: F, t6611: F, t6614: F, t6617: F, t6619: F, t6622: F, t6624: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7747 = t2093 * t2623;
    let t7748 = t166 * t7747;
    let t7750 = t161 * t7748 / 10.0;
    let t7752 = t831 * t2625 / 10.0;
    let t7754 = t2592 * t824 / 10.0;
    let t7756 = t802 * t2631 / 5.0;
    let t7758 = 2.0 / 15.0 * t6611;
    let t7759 = 2.0 / 15.0 * t6614;
    let t7760 = t6617 / 15.0;
    let t7761 = 2.0 / 15.0 * t6619;
    let t7762 = t6622 / 15.0;
    let t7763 = 2.0 / 15.0 * t6624;
    (t7747, t7748, t7750, t7752, t7754, t7756, t7758, t7759, t7760, t7761, t7762, t7763)
}
