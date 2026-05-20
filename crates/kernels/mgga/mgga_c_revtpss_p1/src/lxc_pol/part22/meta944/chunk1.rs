//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3181/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3181<F: Float>(t12948: F, t17377: F, t17361: F, t3708: F, t17290: F, t3678: F, t3625: F, t44250: F, t5401: F, t127: F, t5277: F, t12866: F, t3630: F) -> (F, F, F, F, F, F) {
    let t58878 = t17377 * t12948;
    let t58882 = t3708 * t17361;
    let t58884 = t17290 * t3678;
    let t58889 = t3625 * t44250 * t5401;
    let t58895 = t127 * t5277;
    let t58897 = t12866 * t58895 * t3630;
    (t58878, t58882, t58884, t58889, t58895, t58897)
}
