//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 697/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk697<F: Float>(t2255: F, t787: F, t2730: F, t384: F, t3643: F, t5785: F, t5801: F, t5852: F, t5855: F, t5860: F, t6975: F, t6978: F, t7002: F, t7005: F, t7008: F, t7009: F, t7013: F) -> (F, F, F) {
    let t7056 = t787 * t2255;
    let t7060 = t2730 * t384;
    let t7065 = -t6975 + t6978 - t3643 - 1.532671111111111 * t5852 + t5855 - t7002 + t7005 + t7008 - t5785 - 3.44851 * t5860 - t7009 + t5801 + t7013;
    (t7056, t7060, t7065)
}
