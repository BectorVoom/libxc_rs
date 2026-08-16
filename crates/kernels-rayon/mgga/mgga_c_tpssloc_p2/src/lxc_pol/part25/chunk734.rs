//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 734/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk734(t213: f64, t776: f64, t221: f64, t2553: f64, t59: f64, t8705: f64, t207: f64, t215: f64, t2570: f64, t782: f64, t2573: f64, t2690: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9564 = t213 * t776;
    let t9566 = t221 * t9564 * t2553;
    let t9569 = t59 * t8705;
    let t9572 = 0.28086419753086419752e-1_f64 * t9569 * t207 * t215;
    let t9573 = t782 * t2570;
    let t9574 = t9573 * t2573;
    let t9576 = t59 * t2690;
    (t9566, t9569, t9572, t9573, t9574, t9576)
}
