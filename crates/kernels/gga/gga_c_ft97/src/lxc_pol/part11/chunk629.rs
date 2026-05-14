//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 629/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk629<F: Float>(t1882: F, t2192: F, t2207: F, t1986: F, t609: F, t2185: F, t605: F, t446: F, t9260: F, t9264: F, t9268: F, t9270: F, t9272: F, t9274: F, t9278: F, t9282: F, t9286: F, t9290: F, t9295: F, t9298: F) -> (F, F, F) {
    let t9300 = t1882 * t2192;
    let t9302 = t1882 * t2207;
    let t9304 = t1986 * t609;
    let t9306 = t2185 * t605 * t9304;
    let t9309 = -t446 * t9260 / 3.0 - t446 * t9264 / 3.0 - t446 * t9268 - 4.0 / 9.0 * t9270 - 4.0 / 9.0 * t9272 + t9274 / 3.0 + 2.0 * t446 * t9278 - t9282 / 3.0 + t446 * t9286 + 2.0 * t446 * t9290 + 2.0 * t446 * t9295 - 4.0 / 27.0 * t9298 - 2.0 / 3.0 * t9300 + 2.0 / 27.0 * t9302 - 2.0 * t446 * t9306;
    (t9304, t9306, t9309)
}
