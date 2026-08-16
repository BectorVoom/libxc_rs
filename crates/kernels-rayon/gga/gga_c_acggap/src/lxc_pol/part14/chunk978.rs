//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 978/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk978(t30402: f64, t30407: f64, t30409: f64, t513: f64, t7447: f64, t8637: f64, t8800: f64, t30219: f64, t8661: f64, t30543: f64, t8446: f64, t30934: f64, t8450: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34590 = t30407 * t30402 * t30409 * t513;
    let t34592 = t7447 * t8637;
    let t34593 = 11.0_f64 / 192.0_f64 * t34592;
    let t34609 = t7447 * t8800;
    let t34610 = 11.0_f64 / 192.0_f64 * t34609;
    let t34611 = t30219 * t8661;
    let t34612 = 0.47172138434406228102e-2_f64 * t34611;
    let t34616 = t30543 * t8446;
    let t34617 = 0.18868855373762491241e-1_f64 * t34616;
    let t34618 = t30934 * t8450;
    (t34590, t34593, t34610, t34612, t34617, t34618)
}
