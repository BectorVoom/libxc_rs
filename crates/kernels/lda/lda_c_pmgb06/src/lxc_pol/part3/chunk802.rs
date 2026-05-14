//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 802/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk802<F: Float>(t3100: F, t350: F, t1437: F, t1530: F, t3105: F, t132: F, t3121: F, t435: F, t161: F, t2944: F, t489: F, t3146: F, t490: F, t1490: F, t1554: F, t1541: F, t1547: F) -> (F, F, F, F, F, F, F, F) {
    let t9217 = t350 * t3100;
    let t9220 = 1.0 / t1437 / t1530;
    let t9225 = t350 * t3105;
    let t9234 = t132 * t435 * t3121;
    let t9237 = t161 * t489 * t2944;
    let t9239 = t3146 * t490;
    let t9242 = t161 * t1554 * t1490;
    let t9259 = t132 * t1547 * t1541;
    (t9217, t9220, t9225, t9234, t9237, t9239, t9242, t9259)
}
