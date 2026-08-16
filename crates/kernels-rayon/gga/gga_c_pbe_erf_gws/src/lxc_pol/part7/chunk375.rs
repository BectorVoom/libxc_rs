//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 375/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk375(t1235: f64, t103: f64, t1251: f64, t1: f64, t120: f64, t485: f64, param_hyb_omega_0: f64) -> (f64, f64, f64, f64) {
    let t1508 = param_hyb_omega_0 * t1235;
    let t1509 = t1508 * t103;
    let t1511 = 0.32478055555555555555e0_f64 * t1509 * t1251;
    let t1513 = t485 * t120 * t1;
    (t1508, t1509, t1511, t1513)
}
