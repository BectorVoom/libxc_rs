//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 820/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk820<F: Float>(t2240: F, t602: F, t2246: F, t599: F, t88: F, t89: F, t90: F, t29: F, t2248: F, t644: F, t2315: F, t606: F, t70: F, t72: F, t1927: F, t2258: F) -> (F, F, F, F, F, F, F, F) {
    let t10298 = t2240 * t602;
    let t10301 = t599 * t2246;
    let t10308 = 1.0 / t90 / t89 / t88;
    let t10309 = t29 * t10308;
    let t10310 = t2248 * t644;
    let t10313 = t644 * t2315;
    let t10317 = t606 * t70 * t72;
    let t10318 = t1927 * t2258;
    (t10298, t10301, t10308, t10309, t10310, t10313, t10317, t10318)
}
