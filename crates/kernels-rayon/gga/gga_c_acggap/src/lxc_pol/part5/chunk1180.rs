//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1180/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1180(t1140: f64, t5636: f64, t330: f64, t5787: f64, t1801: f64, t3570: f64, t1165: f64, t13081: f64, t13088: f64, t13090: f64, t13851: f64, t16553: f64, t16557: f64, t16563: f64, t16569: f64, t16575: f64, t1884: f64, t3196: f64) -> f64 {
    let t21433 = t1140 * t5636;
    let t21435 = t330 * t5787;
    let t21440 = t3570 * t1801;
    let t21442 = -0.16006300097412701803e-1_f64 * t16553 + 0.85748036236139473944e-3_f64 * t16557 + 0.34299214494455789578e-2_f64 * t16563 + 0.17149607247227894789e-2_f64 * t16569 + 0.51448821741683684366e-1_f64 * t13851 * t1165 * t1884 * t3196 - 0.80031500487063509016e-2_f64 * t16575 + 7.0_f64 / 144.0_f64 * t21433 - 7.0_f64 / 144.0_f64 * t21435 - 0.25724410870841842183e-2_f64 * t13081 - 0.32012600194825403606e-1_f64 * t13088 + 0.32012600194825403606e-1_f64 * t13090 - 35.0_f64 / 108.0_f64 * t21440;
    t21442
}
