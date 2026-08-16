//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 848/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk848(t17189: f64, t2222: f64, t2221: f64, t17151: f64, t17155: f64, t17158: f64, t17161: f64, t17165: f64, t17170: f64, t17174: f64, t17178: f64, t17183: f64, t17186: f64, t1901: f64, t446: f64, t9270: f64, t9272: f64, t9298: f64, t9321: f64) -> f64 {
    let t17190 = t2222 * t17189;
    let t17191 = t2221 * t17190;
    let t17194 = -4.0_f64 / 27.0_f64 * t9270 - 4.0_f64 / 27.0_f64 * t9272 - 4.0_f64 / 81.0_f64 * t9298 - 2.0_f64 / 3.0_f64 * t446 * t17151 - t446 * t17155 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t17158 + 4.0_f64 / 9.0_f64 * t1901 * t17161 - 4.0_f64 / 27.0_f64 * t1901 * t17165 + 4.0_f64 / 27.0_f64 * t9321 - 2.0_f64 / 3.0_f64 * t446 * t17170 - t446 * t17174 / 3.0_f64 - t446 * t17178 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t17183 + 2.0_f64 / 9.0_f64 * t1901 * t17186 + 2.0_f64 / 9.0_f64 * t1901 * t17191;
    t17194
}
