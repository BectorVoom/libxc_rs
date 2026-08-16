//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 706/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk706(t13492: f64, t2508: f64, t3616: f64, t954: f64, t2936: f64, t3451: f64, t13183: f64, t11627: f64, t7296: f64, t883: f64, t943: f64, t11603: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13494 = 0.92286314761706691403e-1_f64 * t2508 * t13492;
    let t13495 = t954 * t3616;
    let t13497 = 0.76905262301422242837e-2_f64 * t2508 * t13495;
    let t13498 = t2936 * t3451;
    let t13501 = 0.1281754371690370714e-2_f64 * t13183;
    let t13503 = t7296 * t883 * t11627;
    let t13504 = t943 * t13503;
    let t13506 = t11603 * t935;
    (t13494, t13495, t13497, t13498, t13501, t13503, t13504, t13506)
}
