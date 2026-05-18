//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1007/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1007<F: Float>(t1235: F, t12963: F, t12640: F, t225: F, t480: F, t12621: F, t482: F, t371: F, t372: F, t12657: F, t3667: F, t3678: F) -> (F, F, F, F, F, F, F) {
    let t12964 = t1235 * t12963;
    let t12966 = t12640 * t225;
    let t12967 = t12966 * t480;
    let t12970 = t482 * t12621;
    let t12972 = t371 * t372 * t12970;
    let t12975 = t12657 * t225;
    let t12976 = t12975 * t480;
    let t12979 = t3667 * t3678;
    (t12964, t12966, t12967, t12972, t12975, t12976, t12979)
}
