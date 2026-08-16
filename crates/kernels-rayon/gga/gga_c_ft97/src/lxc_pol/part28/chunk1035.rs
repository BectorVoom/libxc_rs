//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1035/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1035(t137105: f64, t137108: f64, t137110: f64, t137124: f64, t137131: f64, t144989: f64, t144994: f64, t145001: f64, t145004: f64, t145008: f64, t145012: f64, t145017: f64, t145022: f64, t145025: f64, t145028: f64, t145032: f64) -> f64 {
    let t145034 = t137105 - 2.0_f64 / 3.0_f64 * t137108 - t137110 / 18.0_f64 + 2.0_f64 * t144989 + 4.0_f64 * t144994 - t137124 / 3.0_f64 + t137131 / 6.0_f64 + 2.0_f64 * t145001 - 2.0_f64 / 3.0_f64 * t145004 - 2.0_f64 * t145008 - 2.0_f64 * t145012 + t145017 / 4.0_f64 + t145022 / 4.0_f64 - 4.0_f64 / 3.0_f64 * t145025 + 2.0_f64 * t145028 + 3.0_f64 / 2.0_f64 * t145032;
    t145034
}
