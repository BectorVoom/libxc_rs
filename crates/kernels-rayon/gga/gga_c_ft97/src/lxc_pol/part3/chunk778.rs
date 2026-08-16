//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 778/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk778(t4608: f64, t8392: f64, t11837: f64, t979: f64, t83: f64, t3238: f64, t3255: f64, t942: f64, t452: f64, t488: f64, t4462: f64, t447: f64, t499: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16083 = t8392 * t4608;
    let t16085 = t11837 * t979;
    let t16086 = t83 * t16085;
    let t16089 = t3238 * t3255;
    let t16090 = t83 * t16089;
    let t16093 = t942 * t3255;
    let t16095 = t452 * t488 * t16093;
    let t16099 = t447 * t499 * t4462;
    (t16083, t16085, t16086, t16089, t16090, t16095, t16099)
}
