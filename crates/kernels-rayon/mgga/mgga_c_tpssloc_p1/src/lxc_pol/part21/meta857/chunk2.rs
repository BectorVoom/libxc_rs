//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3110/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3110(t63398: f64, t63400: f64, t63404: f64, t63408: f64, t63412: f64, t63417: f64, t63422: f64, t64074: f64, t64076: f64, t64079: f64, t64082: f64, t64085: f64, t64087: f64, t64089: f64, t64092: f64) -> f64 {
    let t64422 = -0.80513333333333333333e0_f64 * t63398 - 0.12077e1_f64 * t63400 + 0.181155e1_f64 * t63404 + 0.72462e1_f64 * t63408 + 0.12077e1_f64 * t63412 + 0.33547222222222222222e0_f64 * t63417 - 0.89459259259259259259e0_f64 * t63422 + 0.73586666666666666667e-1_f64 * t64074 + 0.22076e0_f64 * t64076 - 0.5519e-1_f64 * t64079 - 0.16557e0_f64 * t64082 - 0.99342e0_f64 * t64085 - 0.44152e0_f64 * t64087 - 0.66228e0_f64 * t64089 + 0.33114e0_f64 * t64092;
    t64422
}
