//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 737/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk737<F: Float>(t2592: F, t815: F, t2584: F, t802: F, t3092: F, t7284: F, t3090: F, t36: F, t3098: F, t1525: F, t1858: F, t2381: F, t1438: F, t453: F, t1863: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7470 = t2592 * t815 / 10.0;
    let t7472 = t802 * t2584 / 10.0;
    let t7477 = t3092 * t7284;
    let t7478 = t3090 * t7477;
    let t7479 = t36 * t7478;
    let t7481 = t3098 * t7284;
    let t7482 = t1525 * t7481;
    let t7483 = t36 * t7482;
    let t7485 = t1858 * t2381;
    let t7486 = t1525 * t7485;
    let t7487 = t36 * t7486;
    let t7489 = t1438 * t7284;
    let t7490 = t453 * t7489;
    let t7491 = t36 * t7490;
    let t7493 = t1863 * t2381;
    let t7494 = t453 * t7493;
    (t7470, t7472, t7477, t7478, t7479, t7481, t7482, t7483, t7485, t7486, t7487, t7489, t7490, t7491, t7493, t7494)
}
