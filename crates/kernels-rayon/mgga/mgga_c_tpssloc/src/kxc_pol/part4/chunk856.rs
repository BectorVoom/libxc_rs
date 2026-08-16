//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 856/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk856(t180: f64, t2511: f64, t9489: f64, t9490: f64, t761: f64, t116: f64, t229: f64, t212: f64, t776: f64, t2586: f64, t597: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9493 = 1.0_f64 / t2511 / t180;
    let t9494 = t9489 * t9490 * t9493;
    let t9496 = 0.10254018858216406658e4_f64 * t761 * t9494;
    let t9523 = t229 * t116;
    let t9524 = t212 * t776;
    let t9525 = t9523 * t9524;
    let t9526 = t2586 * t9525;
    let t9533 = 1.0_f64 / t60 / t597;
    (t9493, t9494, t9496, t9523, t9526, t9533)
}
