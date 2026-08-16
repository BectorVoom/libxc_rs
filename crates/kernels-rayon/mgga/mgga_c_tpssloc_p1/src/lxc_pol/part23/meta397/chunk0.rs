//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1203/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1203(t16616: f64, t2528: f64, t212: f64, t5544: f64, t5527: f64, t5555: f64, t9541: f64, t41008: f64, t5550: f64, t16783: f64, t41196: f64, t16791: f64, t9546: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t59028 = t16616 * t2528;
    let t59135 = t212 * t5544;
    let t59162 = t212 * t5527;
    let t59195 = t9541 * t5555;
    let t59204 = t41008 * t5550;
    let t59206 = t41196 * t16783;
    let t59218 = t9546 * t16791;
    (t59028, t59135, t59162, t59195, t59204, t59206, t59218)
}
