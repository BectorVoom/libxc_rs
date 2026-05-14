//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1171/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1171<F: Float>(t13713: F, t2470: F, t3198: F, t13719: F, t13721: F, t13104: F, t835: F, t1977: F, t5305: F, t1847: F, t1980: F, t1983: F, t1972: F, t4605: F, t5322: F, t6268: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17680 = 8.0 / 405.0 * t13713;
    let t17682 = t3198 * t2470 / 27.0;
    let t17683 = 8.0 / 135.0 * t13719;
    let t17684 = 4.0 / 81.0 * t13721;
    let t17686 = 2.0 / 45.0 * t13104 * t835;
    let t17688 = 4.0 / 45.0 * t5305 * t1977;
    let t17691 = 8.0 / 45.0 * t1847 * t1980 * t1983;
    let t17693 = 2.0 / 45.0 * t1972 * t4605;
    let t17695 = 8.0 / 45.0 * t6268 * t5322;
    (t17680, t17682, t17683, t17684, t17686, t17688, t17691, t17693, t17695)
}
