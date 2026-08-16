//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1157/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1157(t13984: f64, t14657: f64, t13875: f64, t13884: f64, t13886: f64, t13895: f64, t14624: f64, t14629: f64, t14634: f64, t14640: f64, t14643: f64, t14649: f64, t14652: f64, t14655: f64, t2408: f64, t3066: f64, t335: f64) -> f64 {
    let t14658 = t14657 * t13984;
    let t14660 = -7.0_f64 / 144.0_f64 * t13875 + t3066 * t14624 / 48.0_f64 + t2408 * t14629 / 48.0_f64 + t14634 / 768.0_f64 + 5.0_f64 / 768.0_f64 * t14640 - t335 * t14643 / 96.0_f64 + 7.0_f64 / 288.0_f64 * t13884 + 7.0_f64 / 288.0_f64 * t13886 - t14649 / 96.0_f64 - t2408 * t14652 / 24.0_f64 + t13895 + 7.0_f64 / 1152.0_f64 * t14655 - t14658 / 96.0_f64;
    t14660
}
