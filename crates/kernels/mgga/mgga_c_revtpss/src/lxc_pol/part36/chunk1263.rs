//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1263/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1263<F: Float>(t17628: F, t7607: F, t3655: F, t8177: F, t3596: F, t8190: F, t1811: F, t7642: F, t8945: F, t26948: F, t29135: F, t3566: F) -> (F, F, F, F, F, F) {
    let t104990 = t7607 * t17628;
    let t104999 = t8177 * t3655;
    let t105090 = t3596 * t8190;
    let t105364 = t7642 * t1811;
    let t105365 = t105364 * t8945;
    let t105420 = t26948 * t29135;
    let t105509 = t3566 * t29135;
    (t104990, t104999, t105090, t105365, t105420, t105509)
}
