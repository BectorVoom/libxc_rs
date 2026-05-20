//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1161/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1161<F: Float>(t11291: F, t11293: F, t11296: F, t11303: F, t11382: F, t11390: F, t11392: F, t11394: F, t11590: F, t11593: F, t11596: F, t11600: F, t11604: F) -> F {
    let t12199 = t11291 + t11293 + t11296 - t11303 + t11382 + t11390 + t11604 - t11392 - t11394 - t11593 + t11596 - t11600 + t11590;
    t12199
}
