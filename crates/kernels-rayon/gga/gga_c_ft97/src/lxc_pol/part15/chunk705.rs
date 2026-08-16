//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 705/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk705(t1787: f64, t20145: f64, t20103: f64, t8291: f64, t20107: f64, t11669: f64, t11720: f64, t16373: f64, t16404: f64, t16406: f64, t16442: f64, t16444: f64, t16446: f64, t462: f64, t8301: f64) -> (f64, f64, f64, f64) {
    let t20372 = t1787 * t20145;
    let t20381 = t8291 * t20103;
    let t20384 = t1787 * t20107;
    let t20387 = t462 * t20372 - 4.0_f64 / 9.0_f64 * t11720 + t16404 - 2.0_f64 * t16406 - 4.0_f64 / 3.0_f64 * t11669 - 2.0_f64 / 3.0_f64 * t16373 - t8301 - 2.0_f64 / 3.0_f64 * t16442 + t16444 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t16446 - 2.0_f64 * t462 * t20381 - 2.0_f64 * t462 * t20384;
    (t20372, t20381, t20384, t20387)
}
