//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 399/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk399<F: Float>(t1450: F, t2107: F, t118: F, t2014: F, t2052: F, t2056: F, t2089: F, t2093: F, t508: F, t569: F, t651: F, t3: F, t117: F, t2055: F, t572: F, t573: F) -> (F, F, F, F, F, F) {
    let t2108 = t2107 * t1450;
    let t2110 = -t118 * t2089 + t2014 * t2108 - t2052 * t508 - 2.0 * t2056 * t651 + t2093 * t569;
    let t2111 = t3 * t2110;
    let t2113 = param_d * t2110;
    let t2115 = t117 * t2055;
    let t2118 = t2113 * t573 + 3.0 * t2115 * t572;
    (t2108, t2110, t2111, t2113, t2115, t2118)
}
