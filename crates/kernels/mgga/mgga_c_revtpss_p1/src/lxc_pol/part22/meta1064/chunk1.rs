//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3813/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3813<F: Float>(t73341: F, t13656: F, t198: F, t46963: F, t46970: F, t6816: F, t73327: F, t73328: F, t73330: F, t73332: F, t73333: F, t73334: F, t73338: F, t73339: F) -> (F, F) {
    let t73342 = F::cast_from(0.10843581300301739842e-1_f64) * t73341;
    let t73343 = F::new(6.0) * t13656 * t198 * t6816 - t46963 + t46970 - t73327 - t73328 + t73330 - t73332 - t73333 + t73334 + t73338 + t73339 + t73342;
    (t73342, t73343)
}
