//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 997/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk997(t8392: f64, t9350: f64, t9355: f64, t144: f64, t167: f64, t1901: f64, t1986: f64, t2185: f64, t2221: f64, t2230: f64, t3434: f64, t3440: f64, t39641: f64, t40262: f64, t40696: f64, t40698: f64, t40700: f64, t40720: f64, t40722: f64, t446: f64, t574: f64, t616: f64, t9007: f64, t9133: f64) -> f64 {
    let t40727 = t8392 * t9350;
    let t40729 = t8392 * t9355;
    let t40731 = 112.0_f64 / 81.0_f64 * t40696 + 112.0_f64 / 81.0_f64 * t40698 + 8.0_f64 / 3.0_f64 * t1901 * t2221 * t3440 * t40700 - 2.0_f64 * t446 * t144 * t39641 - 4.0_f64 / 3.0_f64 * t446 * t574 * t616 * t9007 - t446 * t574 * t167 * t40262 / 3.0_f64 + 4.0_f64 * t446 * t2185 * t2230 * t1986 + 8.0_f64 / 9.0_f64 * t40720 + 8.0_f64 / 3.0_f64 * t1901 * t9133 * t3434 * t40722 - 4.0_f64 / 9.0_f64 * t40727 - 4.0_f64 / 9.0_f64 * t40729;
    t40731
}
