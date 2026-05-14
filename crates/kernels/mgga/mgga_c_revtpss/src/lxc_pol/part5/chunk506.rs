//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 506/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk506<F: Float>(t112: F, t2289: F, t625: F, t666: F, t111: F, t654: F, t99: F, t107: F, t200: F, t202: F, t205: F, t262: F, t705: F, t716: F, t198: F, t206: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2335 = 11.0 / 9.0 * t2289 * t112;
    let t2336 = t625 * t666;
    let t2339 = 1.0 / t654 / t111;
    let t2349 = 1.0 / t99;
    let t2357 = 1.0 / t107;
    let t2375 = 1.0 / t200;
    let t2382 = 1.0 / t202;
    let t2393 = t205 * t262;
    let t2398 = t705 * t716;
    let t2403 = t198 * t206;
    (t2335, t2336, t2339, t2349, t2357, t2375, t2382, t2393, t2398, t2403)
}
