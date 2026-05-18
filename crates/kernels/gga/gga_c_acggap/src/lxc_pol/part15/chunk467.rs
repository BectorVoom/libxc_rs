//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 467/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk467<F: Float>(t105: F, t2248: F, t469: F, t495: F, t2001: F, t532: F, t1501: F, t336: F, t570: F, t525: F, t599: F, t1181: F) -> (F, F, F, F, F, F, F) {
    let t2249 = t105 * t2248;
    let t2254 = t469 * t495;
    let t2258 = t2001 * t532;
    let t2260 = t336 * t1501;
    let t2261 = t570 * t2260;
    let t2263 = t599 * t525;
    let t2264 = t1181 * t2263;
    (t2249, t2254, t2258, t2260, t2261, t2263, t2264)
}
