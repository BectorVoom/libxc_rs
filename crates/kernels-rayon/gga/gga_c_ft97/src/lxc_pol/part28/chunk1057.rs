//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1057/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1057(t145585: f64, t27: f64, t370: f64, t89: f64, t34507: f64, t375: f64, t144813: f64, t38262: f64, t446: f64, t144801: f64, t7824: f64, t32063: f64, t34385: f64, t7238: f64) -> (f64, f64, f64, f64, f64) {
    let t145588 = t89 * t27 * t370 * t145585;
    let t145590 = t89 * t375 * t34507;
    let t145595 = t446 * t38262 * t144813;
    let t145598 = t446 * t7824 * t144801;
    let t145601 = t7238 * t32063 * t34385;
    (t145588, t145590, t145595, t145598, t145601)
}
