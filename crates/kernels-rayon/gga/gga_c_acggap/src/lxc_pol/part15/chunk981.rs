//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 981/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk981(t30407: f64, t30418: f64, t31102: f64, t513: f64, t30402: f64, t30409: f64, t7447: f64, t8637: f64, t8800: f64, t30219: f64, t8661: f64, t30543: f64, t8446: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34586 = t30407 * t30418 * t31102 * t513;
    let t34590 = t30407 * t30402 * t30409 * t513;
    let t34592 = t7447 * t8637;
    let t34609 = t7447 * t8800;
    let t34611 = t30219 * t8661;
    let t34616 = t30543 * t8446;
    (t34586, t34590, t34592, t34609, t34611, t34616)
}
