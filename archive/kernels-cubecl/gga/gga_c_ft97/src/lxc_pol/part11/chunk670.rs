//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 670/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk670<F: Float>(t1882: F, t2207: F, t1986: F, t609: F, t2185: F, t605: F, t446: F, t9260: F, t9264: F, t9268: F, t9270: F, t9272: F, t9274: F, t9278: F, t9282: F, t9286: F, t9290: F, t9295: F, t9298: F, t9300: F) -> (F, F, F) {
    let t9302 = t1882 * t2207;
    let t9304 = t1986 * t609;
    let t9306 = t2185 * t605 * t9304;
    let t9309 = -t446 * t9260 / F::cast_from(3.0_f64) - t446 * t9264 / F::cast_from(3.0_f64) - t446 * t9268 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t9270 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t9272 + t9274 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t446 * t9278 - t9282 / F::cast_from(3.0_f64) + t446 * t9286 + F::cast_from(2.0_f64) * t446 * t9290 + F::cast_from(2.0_f64) * t446 * t9295 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t9298 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9300 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t9302 - F::cast_from(2.0_f64) * t446 * t9306;
    (t9304, t9306, t9309)
}
