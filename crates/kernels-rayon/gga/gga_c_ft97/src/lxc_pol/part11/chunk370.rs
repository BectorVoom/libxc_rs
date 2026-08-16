//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 370/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk370(t108: f64, t1538: f64, t1761: f64, t1821: f64, t1826: f64, t1854: f64, t1920: f64, t1922: f64, t438: f64, t497: f64, t88: f64, t1580: f64) -> (f64, f64) {
    let t1927 = -t108 * t1538 - t108 * t1761 - t1920 * t88 - 2.0_f64 * t438 * t497 - 2.0_f64 * t1821 - 4.0_f64 * t1826 + 4.0_f64 * t1854 + 2.0_f64 * t1922;
    let t1934 = -t1580;
    (t1927, t1934)
}
