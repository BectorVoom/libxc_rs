//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2103/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2103<F: Float>(t104480: F, t1243: F, t2149: F, t1811: F, t7642: F, t8945: F, t3596: F, t13181: F, t7635: F, t1209: F, t26948: F, t29135: F) -> (F, F, F, F, F, F) {
    let t105354 = t2149 * t104480 * t1243;
    let t105364 = t7642 * t1811;
    let t105365 = t105364 * t8945;
    let t105383 = t2149 * t104480 * t3596;
    let t105403 = t7635 * t13181;
    let t105404 = t1209 * t105403;
    let t105409 = t7642 * t105403;
    let t105420 = t26948 * t29135;
    (t105354, t105365, t105383, t105404, t105409, t105420)
}
