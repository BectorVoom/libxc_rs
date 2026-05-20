//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3850/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3850<F: Float>(t39419: F, t39422: F, t46289: F, t46297: F, t46963: F, t73314: F, t73315: F, t73316: F, t73317: F, t73322: F, t73327: F, t73328: F, t73330: F, t73332: F, t73333: F, t73334: F, t73338: F) -> F {
    let t74099 = -t73314 + t73315 - t73316 + t46289 - t46297 - t39419 - t39422 - t73317 + t73322 - t73327 - t73328 + t73330 - t73332 - t73333 + t73334 - t46963 + t73338;
    t74099
}
