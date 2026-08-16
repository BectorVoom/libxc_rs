//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 781/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk781(t2662: f64, t7448: f64, t2672: f64, t769: f64, t123: f64, t549: f64, t7451: f64, t2441: f64, t2477: f64, t2471: f64, t827: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7491 = t2662 * t7448;
    let t7492 = t2672 * t769;
    let t7493 = t549 * t123;
    let t7494 = t7492 * t7493;
    let t7495 = t7451 * t7494;
    let t7499 = 0.51947267698127589899e2_f64 * t2441 * t2477;
    let t7501 = 1.0_f64 / t2471 / t827;
    (t7491, t7492, t7493, t7494, t7495, t7499, t7501)
}
