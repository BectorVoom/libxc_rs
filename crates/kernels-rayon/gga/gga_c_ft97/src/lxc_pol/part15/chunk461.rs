//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 461/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk461(t140: f64, t4710: f64, t550: f64, t133: f64, t2001: f64, t4675: f64, t4677: f64, t4700: f64, t4704: f64) -> (f64, f64) {
    let t141 = 0.1e-59_f64 < t140;
    let t4711 = t550 * t4710;
    let t4712 = t133 * t4711;
    let t4714 = piecewise3(t141, -4.0_f64 * t2001 * t4677 + 2.0_f64 * t4675 + 2.0_f64 * t4700 + 2.0_f64 * t4704 - t4712, 0.0_f64);
    (t4711, t4714)
}
