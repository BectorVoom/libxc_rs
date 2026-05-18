//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 889/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk889<F: Float>(t1284: F, t1811: F, t1209: F, t1789: F, t371: F, t676: F, t1235: F, t1769: F, t3565: F, t225: F, t480: F, t1804: F, t3655: F) -> (F, F, F, F, F, F, F) {
    let t17191 = t1284 * t1811;
    let t17192 = t1209 * t17191;
    let t17303 = t371 * t676 * t1789;
    let t17304 = t1235 * t17303;
    let t17306 = t1769 * t3565;
    let t17307 = t17306 * t225;
    let t17308 = t17307 * t480;
    let t17340 = t1804 * t3655;
    (t17192, t17303, t17304, t17306, t17307, t17308, t17340)
}
