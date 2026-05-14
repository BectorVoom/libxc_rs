//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 711/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk711<F: Float>(t5186: F, t5196: F, t5207: F, t5209: F, t6540: F, t6543: F, t6547: F, t6549: F, t6553: F, t6558: F, t6564: F, t6565: F, t6566: F, t6567: F, t7205: F, t183: F, t6716: F) -> (F, F) {
    let t7207 = t6540 + t6543 + t6547 + t6549 + t6553 + t6558 + t6564 - t6565 - t6566 - t6567 + 4.0 / 3.0 * t7205 + t5186 + t5196 + t5207 + t5209;
    let t7209 = t6716 * t183;
    (t7207, t7209)
}
