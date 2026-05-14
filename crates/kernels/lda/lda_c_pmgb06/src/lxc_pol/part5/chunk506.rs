//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 506/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk506<F: Float>(t107: F, t110: F, t122: F, t1658: F, t1672: F, t1741: F, t1796: F, t1804: F, t1813: F, t199: F, t202: F, t2122: F, t2407: F, t2422: F, t2454: F, t2659: F, t795: F, t84: F, t868: F) -> (F,) {
    let t2667 = -t1658 + 0.1675256410710088 * t1796 + 0.1675256410710088 * t1804 - 0.0837628205355044 * t2454 * t199 - 0.1675256410710088 * t795 * t868 - 0.0837628205355044 * t84 * t2422 - t1672 + 0.039794582218349216 * t1813 - 0.011938374665504766 * t122 * t202 * t2659 + t1741 - 1.1389037339096726 * t2122 + 0.42708890021612717 * t107 * t110 * t2407;
    (t2667,)
}
