//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3813/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3813(t73341: f64, t13656: f64, t198: f64, t46963: f64, t46970: f64, t6816: f64, t73327: f64, t73328: f64, t73330: f64, t73332: f64, t73333: f64, t73334: f64, t73338: f64, t73339: f64) -> (f64, f64) {
    let t73342 = 0.10843581300301739842e-1_f64 * t73341;
    let t73343 = 6.0_f64 * t13656 * t198 * t6816 - t46963 + t46970 - t73327 - t73328 + t73330 - t73332 - t73333 + t73334 + t73338 + t73339 + t73342;
    (t73342, t73343)
}
