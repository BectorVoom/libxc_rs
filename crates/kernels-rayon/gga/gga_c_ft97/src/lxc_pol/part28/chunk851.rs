//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 851/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk851(t28: f64, t34584: f64, t1337: f64, t6455: f64, t6412: f64, t7150: f64, t1308: f64, t925: f64, t356: f64, t461: f64, t6520: f64, t6454: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34585 = t28 * t34584;
    let t34588 = t6455 * t1337;
    let t34589 = t28 * t34588;
    let t34592 = t6412 * t7150;
    let t34595 = t1308 * t925;
    let t34596 = t356 * t34595;
    let t34601 = t461 * t6520;
    let t34607 = t72 * t6454;
    (t34585, t34588, t34589, t34592, t34595, t34596, t34601, t34607)
}
