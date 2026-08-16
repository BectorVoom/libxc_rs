//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1274/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1274<F: Float>(t1264: F, t24240: F, t247: F, t1794: F, t3603: F, t20800: F, t3720: F, t471: F, t6573: F, t1250: F, t17661: F, t6639: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
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
    (t24726, t24729, t24730, t24731, t24734, t24735, t24736, t24739, t24740, t24741, t24744)
}
