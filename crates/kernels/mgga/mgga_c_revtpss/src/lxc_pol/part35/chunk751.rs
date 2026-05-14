//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 751/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk751<F: Float>(t1789: F, t371: F, t676: F, t1235: F, t1769: F, t3565: F, t225: F, t480: F, t1804: F, t3655: F, t1786: F, t11262: F, t1796: F, t1247: F, t1770: F, t3140: F) -> (F, F, F, F, F, F, F, F) {
    let t17303 = t371 * t676 * t1789;
    let t17304 = t1235 * t17303;
    let t17306 = t1769 * t3565;
    let t17307 = t17306 * t225;
    let t17308 = t17307 * t480;
    let t17340 = t1804 * t3655;
    let t17342 = t1786 * t3655;
    let t17361 = t11262 * t1796;
    let t17362 = t1247 * t17361;
    let t17376 = t1770 * t3140;
    (t17304, t17306, t17307, t17308, t17340, t17342, t17362, t17376)
}
