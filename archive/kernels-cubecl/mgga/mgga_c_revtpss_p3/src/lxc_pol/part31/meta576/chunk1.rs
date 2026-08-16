//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1994/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1994<F: Float>(t25375: F, t93285: F, t1957: F, t2718: F, t25386: F, t25331: F, t25365: F, t25372: F, t93280: F, t786: F, t860: F, t25410: F) -> (F, F, F, F, F, F, F) {
    let t93286 = t25375 * t93285;
    let t93301 = t1957 * t2718;
    let t93302 = t25386 * t93301;
    let t93306 = t25365 * t25331;
    let t93314 = t25372 * t93301;
    let t93317 = t25386 * t93280;
    let t93320 = t786 * t860;
    let t93321 = t93320 * t25410;
    (t93286, t93302, t93306, t93314, t93317, t93320, t93321)
}
