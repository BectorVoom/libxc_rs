//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 695/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk695(t3650: f64, t7301: f64, t943: f64, t11613: f64, t2624: f64, t2508: f64, t3616: f64, t954: f64, t13183: f64, t11627: f64, t7296: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13489 = t3650 * t7301;
    let t13490 = t943 * t13489;
    let t13492 = t11613 * t2624;
    let t13494 = 0.92286314761706691403e-1_f64 * t2508 * t13492;
    let t13495 = t954 * t3616;
    let t13497 = 0.76905262301422242837e-2_f64 * t2508 * t13495;
    let t13501 = 0.1281754371690370714e-2_f64 * t13183;
    let t13503 = t7296 * t883 * t11627;
    (t13489, t13490, t13492, t13494, t13495, t13497, t13501, t13503)
}
