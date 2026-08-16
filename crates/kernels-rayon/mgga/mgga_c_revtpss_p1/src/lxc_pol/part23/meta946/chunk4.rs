//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3120/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3120(t43888: f64, t56236: f64, t58607: f64, t58609: f64, t58624: f64, t68332: f64, t68334: f64, t68336: f64, t68389: f64, t68399: f64, t68454: f64, t68456: f64, t81224: f64, t81228: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t81242: f64, t81245: f64) -> f64 {
    let t82111 = 0.11415555555555555555e-1_f64 * t68332 + 0.2283111111111111111e-1_f64 * t68334 + 0.6849333333333333333e-1_f64 * t68336 - t58607 + t58609 + 0.30822e0_f64 * t81224 + 0.17123333333333333333e-1_f64 * t81228 - 0.63419753086419753083e-2_f64 * t81230 + 0.2283111111111111111e-1_f64 * t81232 - 0.34246666666666666667e-1_f64 * t81234 - 0.57077777777777777777e-2_f64 * t81236 + t58624 - 0.5327259259259259259e-1_f64 * t56236 - 0.17123333333333333333e-1_f64 * t68389 + 0.4566222222222222222e-1_f64 * t68399 + 0.57077777777777777775e-1_f64 * t81242 - 0.20547999999999999999e0_f64 * t81245 - 0.17757530864197530864e-1_f64 * t43888 - 0.6849333333333333333e-1_f64 * t68454 - 0.10274e0_f64 * t68456;
    t82111
}
