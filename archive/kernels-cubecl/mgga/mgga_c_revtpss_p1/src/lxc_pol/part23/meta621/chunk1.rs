//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2303/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2303<F: Float>(t1250: F, t24739: F, t3720: F, t17661: F, t6639: F, t1794: F, t6587: F) -> (F, F, F, F) {
    let t24740 = t24739 * t1250;
    let t24741 = t3720 * t24740;
    let t24744 = t17661 * t6639;
    let t24751 = t6587 * t1794;
    (t24740, t24741, t24744, t24751)
}
