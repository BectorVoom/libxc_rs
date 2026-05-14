//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 187/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk187<F: Float>(t841: F, t846: F, t848: F, t812: F, t833: F, t834: F, t836: F, t839: F) -> (F, F) {
    let t849 = t841 * t846 * t848;
    let t852 = t812 + t833 - 0.18311555036753159941e-3 * t834 * t836 - 0.58482233974552040708e0 * t839 * t849;
    (t849, t852)
}
