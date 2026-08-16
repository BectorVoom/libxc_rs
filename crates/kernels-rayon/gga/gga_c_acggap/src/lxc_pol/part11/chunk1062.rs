//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1062/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1062(t30402: f64, t30407: f64, t30409: f64, t513: f64, t7447: f64, t8637: f64, t4578: f64, t7450: f64, t7815: f64, t4483: f64, t2030: f64, t4582: f64) -> (f64, f64, f64, f64, f64) {
    let t34590 = t30407 * t30402 * t30409 * t513;
    let t34592 = t7447 * t8637;
    let t34593 = 11.0_f64 / 192.0_f64 * t34592;
    let t34595 = t7450 * t7815 * t4578;
    let t34598 = t7450 * t7815 * t4483;
    let t34601 = t2030 * t7815 * t4582;
    (t34590, t34593, t34595, t34598, t34601)
}
