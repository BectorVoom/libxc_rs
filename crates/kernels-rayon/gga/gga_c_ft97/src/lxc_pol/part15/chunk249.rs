//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 249/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk249(t1053: f64, t605: f64, t144: f64, t1026: f64, t1030: f64, t1047: f64, t28: f64, t446: f64, t568: f64, t89: f64, t1045: f64, t160: f64) -> (f64, f64, f64, f64) {
    let t1054 = t605 * t1053;
    let t1055 = t144 * t1054;
    let t1058 = -t568 - t446 * t1026 / 9.0_f64 - t446 * t1030 / 3.0_f64 + t89 * t28 * t1047 / 3.0_f64 - t446 * t1055 / 3.0_f64;
    let t1060 = t1045 * t160;
    (t1054, t1055, t1058, t1060)
}
