//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 515/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk515(t3874: f64, t4002: f64, t258: f64, t3951: f64, t1137: f64, t1173: f64, t247: f64, t263: f64, t3683: f64, t3827: f64, t3865: f64, t3973: f64, t3978: f64, t3982: f64, t719: f64, t771: f64) -> (f64, f64, f64) {
    let t4003 = t3874 + t4002;
    let t4005 = t3951 * t258;
    let t4011 = -t1137 * t771 - t1173 * t719 - t247 * t4003 - t263 * t3683 - t263 * t3827 + 4.0_f64 * t3865 - 2.0_f64 * t3973 - 2.0_f64 * t3978 - 2.0_f64 * t3982 + 2.0_f64 * t4005;
    (t4003, t4005, t4011)
}
