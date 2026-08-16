//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 701/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk701(t1008: f64, t2057: f64, t550: f64, t1995: f64, t527: f64, t11260: f64, t1018: f64, t1636: f64, t89: f64, t1026: f64, t8232: f64, t1882: f64, t3463: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12401 = t2057 * t1008;
    let t12448 = t550 * t1008;
    let t12449 = t1995 * t12448;
    let t12452 = t527 * t12448;
    let t12527 = 0.22226000364197530866e-1_f64 * t11260;
    let t12571 = t89 * t1636 * t1018;
    let t12617 = t8232 * t1026;
    let t12620 = 2.0_f64 / 27.0_f64 * t1882 * t3463;
    (t12401, t12449, t12452, t12527, t12571, t12617, t12620)
}
