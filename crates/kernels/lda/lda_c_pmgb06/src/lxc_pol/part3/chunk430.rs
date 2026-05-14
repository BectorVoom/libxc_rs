//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 430/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk430<F: Float>(t107: F, t1180: F, t290: F, t410: F, t701: F, t110: F, t1126: F, t1200: F, t122: F, t1338: F, t1658: F, t1659: F, t1661: F, t1672: F, t1674: F, t1735: F, t199: F, t202: F, t399: F, t566: F, t84: F) -> (F, F, F) {
    let t1741 = 1.328721022894618 * t107 * t1180 * t290;
    let t1743 = t107 * t410 * t701;
    let t1748 = -t1658 + 0.1675256410710088 * t1659 + 0.1675256410710088 * t1661 - 0.0837628205355044 * t1338 * t199 - 0.1675256410710088 * t399 * t566 - 0.0837628205355044 * t84 * t1200 - t1672 + 0.039794582218349216 * t1674 - 0.011938374665504766 * t122 * t202 * t1735 + t1741 - 1.1389037339096726 * t1743 + 0.42708890021612717 * t107 * t110 * t1126;
    (t1741, t1743, t1748)
}
