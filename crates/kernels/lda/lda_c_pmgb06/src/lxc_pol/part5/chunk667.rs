//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 667/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk667<F: Float>(t477: F, t6554: F, t1966: F, t439: F, t1601: F, t497: F, t764: F, t851: F, t529: F, t5068: F, t3156: F, t3214: F, t3224: F, t5186: F, t6526: F, t6530: F, t6532: F, t6535: F, t6538: F, t6540: F, t6543: F, t6547: F, t6549: F, t6553: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6555 = t6554 * t477;
    let t6556 = t1966 * t6555;
    let t6558 = t439 * t6556 / 15.0;
    let t6559 = t1601 * t497;
    let t6560 = t764 * t851;
    let t6561 = t6560 * t529;
    let t6562 = t6559 * t6561;
    let t6564 = 4.0 / 45.0 * t5068 * t6562;
    let t6565 = t3156 / 135.0;
    let t6566 = 2.0 / 405.0 * t3214;
    let t6567 = 2.0 / 405.0 * t3224;
    let t6568 = t6526 + t6530 + t6532 + t6535 - t6538 + t6540 + t6543 + t6547 + t6549 + t6553 + t6558 + t6564 - t6565 - t6566 - t6567 + t5186;
    (t6555, t6556, t6558, t6559, t6560, t6562, t6564, t6565, t6566, t6567, t6568)
}
