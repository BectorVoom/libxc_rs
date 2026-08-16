//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1009/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1009<F: Float>(t3156: F, t3172: F, t3150: F, t11997: F, t3144: F, t3141: F, t11678: F, t4910: F, t3117: F, t1032: F, t3043: F, t1040: F) -> (F, F, F, F, F, F) {
    let t12009 = t3172 * t3156;
    let t12010 = t3150 * t12009;
    let t12012 = t3144 * t11997;
    let t12013 = t3141 * t12012;
    let t12016 = t11678 * t4910;
    let t12017 = t3117 * t12016;
    let t12020 = t3043 * t1032;
    let t12021 = t12020 * t1040;
    (t12009, t12010, t12013, t12017, t12020, t12021)
}
