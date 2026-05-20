//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1867/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1867<F: Float>(t25898: F, t7527: F, t94849: F, t94383: F, t96221: F, t2453: F, t26264: F, t9676: F, t10073: F, t1444: F, t2102: F, t25929: F) -> (F, F, F, F, F) {
    let t96506 = t94849 * t25898 * t7527;
    let t96510 = t94383 * t96221;
    let t96515 = t2453 * t26264;
    let t96516 = t96515 * t9676;
    let t96546 = t10073 * t25929 * t2102 * t1444;
    (t96506, t96510, t96515, t96516, t96546)
}
