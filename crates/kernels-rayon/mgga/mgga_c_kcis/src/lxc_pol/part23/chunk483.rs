//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 483/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk483(t1378: f64, t4007: f64, t286: f64, t1368: f64, t3969: f64, t3972: f64, t3975: f64, t3981: f64, t3986: f64, t3991: f64, t3995: f64, t4003: f64, t493: f64) -> (f64, f64, f64) {
    let t4008 = t1378 * t4007;
    let t4009 = t286 * t4008;
    let t4012 = -t3969 + t3972 / 432.0_f64 - t3975 / 144.0_f64 + t1368 * t3981 / 216.0_f64 - t1368 * t3986 / 144.0_f64 - t1368 * t3991 / 144.0_f64 + t1368 * t3995 / 288.0_f64 + t493 * t4003 / 48.0_f64 - t493 * t4009 / 96.0_f64;
    (t4008, t4009, t4012)
}
