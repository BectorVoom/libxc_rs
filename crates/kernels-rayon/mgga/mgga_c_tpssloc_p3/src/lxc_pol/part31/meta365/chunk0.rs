//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1290/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1290(t16562: f64, t16574: f64, t145: f64, t185: f64, t5520: f64, t751: f64, t157: f64, t182: f64, t12861: f64, t4119: f64, t4315: f64, t5392: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16575 = t16562 + t16574;
    let t16576 = t145 * t16575;
    let t16577 = t16576 * t185;
    let t16578 = t5520 * t751;
    let t16579 = t16575 * t157;
    let t16581 = 0.19751673498613801407e-1_f64 * t16579 * t182;
    let t16582 = 2.0_f64 * t12861;
    let t16583 = t4315 * t4119;
    let t16586 = t751 * t5392;
    (t16577, t16578, t16581, t16582, t16583, t16586)
}
