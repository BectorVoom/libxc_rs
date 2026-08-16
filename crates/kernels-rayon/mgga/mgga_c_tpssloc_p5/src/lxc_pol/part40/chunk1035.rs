//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1035/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1035(t40: f64, t12943: f64, t4101: f64, t4205: f64, t4202: f64, t16558: f64, t185: f64, t707: f64, t5392: f64, t634: f64, t5398: f64, t75: f64, t3966: f64, t4104: f64, t607: f64, t767: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t16629 = 0.23392894490538584828e1_f64 * t12943;
    let t16630 = t4205 * t4101;
    let t16631 = 8.0_f64 * t16630;
    let t16633 = 8.0_f64 * t4205 * t4202;
    let t16634 = t185 * t16558;
    let t16636 = 4.0_f64 * t707 * t16634;
    let t16637 = t634 * t5392;
    let t16642 = t75 * t5398;
    let t16648 = piecewise3(t146, 0.0_f64, 8.0_f64 / 27.0_f64 * t16637 * t607 - 4.0_f64 / 9.0_f64 * t4104 * t3966 - 2.0_f64 / 9.0_f64 * t16642 * t607 + 2.0_f64 / 3.0_f64 * t767 * t16558);
    (t16629, t16631, t16633, t16636, t16648)
}
