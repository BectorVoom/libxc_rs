//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 992/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk992<F: Float>(t1264: F, t24240: F, t247: F, t1794: F, t3603: F, t20800: F, t3720: F, t471: F, t6573: F, t1250: F, t17661: F, t6639: F, t6587: F, t1715: F, t20809: F, t1042: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24726 = t247 * t1264 * t24240;
    let t24729 = t3603 * t1794;
    let t24730 = t20800 * t24729;
    let t24731 = t3720 * t24730;
    let t24734 = t1794 * t471;
    let t24735 = t20800 * t24734;
    let t24736 = t3720 * t24735;
    let t24739 = t6573 * t1794;
    let t24740 = t24739 * t1250;
    let t24741 = t3720 * t24740;
    let t24744 = t17661 * t6639;
    let t24751 = t6587 * t1794;
    let t24752 = t24751 * t1250;
    let t24753 = t3720 * t24752;
    let t24758 = t20809 * t1715;
    let t24759 = t1042 * t24758;
    (t24726, t24731, t24736, t24739, t24741, t24744, t24751, t24753, t24759)
}
