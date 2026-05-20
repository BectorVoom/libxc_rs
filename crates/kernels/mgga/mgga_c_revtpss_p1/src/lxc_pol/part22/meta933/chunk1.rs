//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3164/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3164<F: Float>(t12916: F, t17743: F, t3718: F, t12881: F, t5391: F, t1222: F, t16720: F, t17471: F, t17753: F, t17755: F, t12800: F, t5378: F) -> (F, F, F, F, F) {
    let t57386 = t3718 * t12916 * t17743;
    let t57421 = t5391 * t12881;
    let t57428 = t1222 * t17471 * t16720;
    let t57435 = t17753 * t12916 * t17755;
    let t57449 = t12800 * t5378;
    (t57386, t57421, t57428, t57435, t57449)
}
