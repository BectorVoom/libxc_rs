//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 426/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk426<F: Float>(t107: F, t200: F, t202: F, t205: F, t262: F, t198: F, t206: F) -> (F, F, F, F, F) {
    let t2357 = 1.0 / t107;
    let t2375 = 1.0 / t200;
    let t2382 = 1.0 / t202;
    let t2393 = t205 * t262;
    let t2403 = t198 * t206;
    (t2357, t2375, t2382, t2393, t2403)
}
