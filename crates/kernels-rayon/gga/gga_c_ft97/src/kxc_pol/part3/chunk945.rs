//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 945/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk945(t4973: f64, t724: f64, t773: f64, t18123: f64, t265: f64, t2594: f64, t4965: f64, t1091: f64, t4005: f64, t4934: f64, t766: f64, t2574: f64, t762: f64) -> (f64, f64, f64, f64, f64) {
    let t18602 = t724 * t773 * t4973;
    let t18606 = t724 * t265 * t18123;
    let t18610 = t2594 * t773 * t4965;
    let t18614 = t724 * t4005 * t1091;
    let t18617 = t4934 * t766;
    let t18619 = t2574 * t762 * t18617;
    (t18602, t18606, t18610, t18614, t18619)
}
