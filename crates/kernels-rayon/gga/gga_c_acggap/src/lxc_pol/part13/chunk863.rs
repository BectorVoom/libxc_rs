//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 863/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk863(t1084: f64, t30148: f64, t30159: f64, t7586: f64, t377: f64, t7779: f64, t606: f64, t7: f64, t7508: f64, t8: f64, t151: f64, t56: f64, t593: f64) -> (f64, f64, f64, f64, f64) {
    let t30162 = t30159 * t7586 * t30148 * t1084;
    let t30169 = t377 * t7779;
    let t30170 = t30169 * t606;
    let t30171 = 0.19812298142450615803e-1_f64 * t30170;
    let t30174 = t7508 * t7;
    let t30176 = 1.0_f64 / t8 / t30174;
    let t30179 = t151 * t593 * t30176 * t56;
    (t30162, t30169, t30171, t30174, t30179)
}
