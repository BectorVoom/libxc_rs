//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 977/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk977(t1701: f64, t19120: f64, t17975: f64, t811: f64, t820: f64, t19100: f64, t800: f64, t19106: f64, t285: f64, t4089: f64, t4092: f64, t4061: f64, t5261: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19121 = t1701 * t19120;
    let t19125 = t1701 * t17975 * t811;
    let t19128 = t17975 * t820;
    let t19129 = t1701 * t19128;
    let t19132 = t800 * t19100;
    let t19135 = t285 * t19106;
    let t19144 = t4092 * t4089;
    let t19147 = t4061 * t5261;
    (t19121, t19125, t19129, t19132, t19135, t19144, t19147)
}
