//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 449/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk449<F: Float>(t7165: F, t7243: F, t7238: F, t7239: F, t1800: F, t1317: F, t28: F, t469: F, t7211: F, t1587: F, t27: F, t89: F, t370: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7244 = t7243 * t7165;
    let t7246 = t7238 * t7239 * t7244;
    let t7248 = t1800 * t7165;
    let t7250 = t1317 * t28 * t7248;
    let t7252 = t469 * t7211;
    let t7254 = t1317 * t28 * t7252;
    let t7256 = t1587 * t7165;
    let t7258 = t89 * t27 * t7256;
    let t7260 = t370 * t7211;
    let t7262 = t89 * t27 * t7260;
    let t7264 = -t7246 / 3.0 + t7250 / 3.0 - t7254 / 6.0 + 2.0 / 3.0 * t7258 - t7262 / 3.0;
    (t7244, t7246, t7248, t7250, t7252, t7254, t7256, t7258, t7260, t7262, t7264)
}
