//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 863/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk863(t9689: f64, t9692: f64, t9695: f64, t9698: f64, t9702: f64, t9704: f64, t9706: f64, t9709: f64, t739: f64, t746: f64, t761: f64, t177: f64, t2508: f64) -> (f64, f64, f64, f64) {
    let t9711 = -0.34523333333333333333e1_f64 * t9689 + 0.23015555555555555556e1_f64 * t9692 - 0.26851481481481481482e1_f64 * t9695 - 0.93932222222222222223e0_f64 * t9698 + 0.73355e-1_f64 * t9702 - 0.14671e0_f64 * t9704 - 0.17116166666666666667e0_f64 * t9706 - 0.36793333333333333333e0_f64 * t9709;
    let t9713 = t739 * t9711 * t746;
    let t9715 = 0.5848223622634646207e0_f64 * t761 * t9713;
    let t9720 = 1.0_f64 / t2508 / t177;
    (t9711, t9713, t9715, t9720)
}
