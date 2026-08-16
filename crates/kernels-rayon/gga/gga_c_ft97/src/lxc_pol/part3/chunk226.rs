//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 226/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk226(t192: f64, t713: f64, t743: f64, t462: f64, t736: f64, t740: f64, t92: f64, t734: f64, t91: f64, t663: f64, t672: f64, t716: f64) -> (f64, f64, f64, f64, f64) {
    let t745 = t192 * t743 * t713;
    let t747 = -t736 - t462 * t740 / 3.0_f64 - t92 * t745;
    let t749 = t91 * t734 * t747;
    let t751 = t663 / 9.0_f64;
    let t754 = t749 / 6.0_f64 - t751 - t672 / 9.0_f64 - t716 / 3.0_f64;
    (t745, t747, t749, t751, t754)
}
