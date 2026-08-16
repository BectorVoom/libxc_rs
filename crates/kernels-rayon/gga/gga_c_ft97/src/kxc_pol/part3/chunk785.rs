//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 785/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk785(t4613: f64, t8392: f64, t11535: f64, t11537: f64, t11549: f64, t11550: f64, t11593: f64, t16147: f64, t16152: f64, t16157: f64, t16162: f64, t16166: f64, t16171: f64, t16174: f64, t16179: f64, t16184: f64, t16188: f64, t1901: f64, t446: f64) -> f64 {
    let t16192 = t8392 * t4613;
    let t16194 = -4.0_f64 / 3.0_f64 * t1901 * t16147 + 4.0_f64 / 9.0_f64 * t1901 * t16152 - 2.0_f64 / 9.0_f64 * t1901 * t16157 - 2.0_f64 / 9.0_f64 * t1901 * t16162 - 2.0_f64 / 3.0_f64 * t1901 * t16166 + 8.0_f64 / 9.0_f64 * t11593 * t16171 + 2.0_f64 / 9.0_f64 * t1901 * t16174 + 2.0_f64 / 9.0_f64 * t1901 * t16179 + 4.0_f64 / 9.0_f64 * t11593 * t16184 + t11535 + t11537 - t446 * t16188 / 3.0_f64 + t11549 - 8.0_f64 / 27.0_f64 * t11550 - 2.0_f64 / 27.0_f64 * t16192;
    t16194
}
