//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1454/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1454<F: Float>(t5284: F, t73: F, t17350: F, t3767: F, t372: F, t5277: F, t1285: F, t12865: F, t15904: F, t3623: F, t13148: F, t3172: F, t5303: F) -> (F, F, F, F, F, F, F) {
    let t17633 = t5284 * t73;
    let t17654 = t3767 * t17350;
    let t17661 = t372 * t5277;
    let t17693 = t1285 * t12865;
    let t17708 = t3623 * t15904;
    let t17709 = t13148 * t17708;
    let t17720 = t3172 * t5303;
    (t17633, t17654, t17661, t17693, t17708, t17709, t17720)
}
