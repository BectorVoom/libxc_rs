//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1165/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1165(t22606: f64, t22609: f64, t18562: f64, t18567: f64, t18571: f64, t18574: f64, t18577: f64, t18580: f64, t18587: f64, t18594: f64, t18599: f64, t18604: f64, t18607: f64) -> (f64, f64, f64) {
    let t48488 = 4.0_f64 * t22606;
    let t48489 = 48.0_f64 * t22609;
    let t48490 = -t18562 + t48488 + t18567 + t18571 - t18574 - t48489 + t18577 + t18580 + t18587 + t18594 + t18599 - t18604 - t18607;
    (t48488, t48489, t48490)
}
