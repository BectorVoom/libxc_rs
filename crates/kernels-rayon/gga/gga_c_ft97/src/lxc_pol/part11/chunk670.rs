//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 670/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk670(t1882: f64, t2207: f64, t1986: f64, t609: f64, t2185: f64, t605: f64, t446: f64, t9260: f64, t9264: f64, t9268: f64, t9270: f64, t9272: f64, t9274: f64, t9278: f64, t9282: f64, t9286: f64, t9290: f64, t9295: f64, t9298: f64, t9300: f64) -> (f64, f64, f64) {
    let t9302 = t1882 * t2207;
    let t9304 = t1986 * t609;
    let t9306 = t2185 * t605 * t9304;
    let t9309 = -t446 * t9260 / 3.0_f64 - t446 * t9264 / 3.0_f64 - t446 * t9268 - 4.0_f64 / 9.0_f64 * t9270 - 4.0_f64 / 9.0_f64 * t9272 + t9274 / 3.0_f64 + 2.0_f64 * t446 * t9278 - t9282 / 3.0_f64 + t446 * t9286 + 2.0_f64 * t446 * t9290 + 2.0_f64 * t446 * t9295 - 4.0_f64 / 27.0_f64 * t9298 - 2.0_f64 / 3.0_f64 * t9300 + 2.0_f64 / 27.0_f64 * t9302 - 2.0_f64 * t446 * t9306;
    (t9304, t9306, t9309)
}
