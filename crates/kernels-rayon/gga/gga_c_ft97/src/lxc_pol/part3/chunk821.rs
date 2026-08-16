//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 821/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk821(t12401: f64, t16762: f64, t4710: f64, t549: f64, t2057: f64, t3355: f64, t3404: f64, t4711: f64, t542: f64, t131: f64, t4673: f64, t139: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16763 = t12401 * t16762;
    let t16769 = t549 * t4710;
    let t16773 = t2057 * t4710;
    let t16777 = t3355 * t3404;
    let t16780 = t542 * t4711;
    let t16785 = t4673 * t131;
    let t16786 = t16785 * t139;
    (t16763, t16769, t16773, t16777, t16780, t16786)
}
