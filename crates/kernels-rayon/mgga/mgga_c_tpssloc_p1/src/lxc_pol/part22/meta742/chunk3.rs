//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2455/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2455(t21456: f64, t28565: f64, t343: f64, t4540: f64, t4546: f64, t48329: f64, t48336: f64, t48339: f64, t48374: f64, t48379: f64, t48382: f64, t48397: f64, t61447: f64, t61472: f64, t61489: f64, t61495: f64, t61557: f64, t61597: f64, t61600: f64, t61602: f64, t973: f64, t984: f64) -> f64 {
    let t69837 = t48329 + 0.27777777777777777777e-3_f64 * t61447 - 0.9259259259259259259e-3_f64 * t48336 - t48339 - 0.83333333333333333331e-3_f64 * t61472 + 0.37037037037037037036e-3_f64 * t61489 - 0.55555555555555555554e-3_f64 * t61495 - 0.55555555555555555554e-3_f64 * t61557 + t48374 - t48379 + t48382 - 0.25e-2_f64 * t973 * t4546 * t28565 * t4540 - 0.83333333333333333332e-3_f64 * t973 * t4546 * t21456 * t984 * t343 - 0.18518518518518518518e-3_f64 * t61597 - 0.24691358024691358024e-3_f64 * t61600 + 0.14814814814814814814e-2_f64 * t61602 + 0.3086419753086419753e-3_f64 * t48397;
    t69837
}
