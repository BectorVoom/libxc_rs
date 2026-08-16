//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 702/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk702(t683: f64, t9596: f64, t92: f64, t9557: f64, t9558: f64, t9560: f64, t9562: f64, t9564: f64, t9574: f64, t9580: f64, t9585: f64, t9589: f64, t9594: f64) -> (f64, f64, f64) {
    let t9597 = t683 * t9596;
    let t9598 = t92 * t9597;
    let t9600 = -t9557 - 4.0_f64 / 9.0_f64 * t9558 + 2.0_f64 / 9.0_f64 * t9560 - 2.0_f64 / 3.0_f64 * t9562 + t9564 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t9574 + 4.0_f64 / 3.0_f64 * t9580 - 2.0_f64 / 3.0_f64 * t9585 - 2.0_f64 * t9589 + 2.0_f64 * t9594 - t9598 / 3.0_f64;
    (t9597, t9598, t9600)
}
