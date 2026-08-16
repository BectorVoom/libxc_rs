//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 884/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk884(t761: f64, t9713: f64, t172: f64, t2448: f64, t763: f64, t177: f64, t2508: f64, t2512: f64, t9490: f64, t2517: f64, t718: f64, t2475: f64, t723: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9715 = 0.5848223622634646207e0_f64 * t761 * t9713;
    let t9716 = t2448 * t172;
    let t9717 = t9716 * t763;
    let t9720 = 1.0_f64 / t2508 / t177;
    let t9722 = t9720 * t9490 * t2512;
    let t9724 = 0.10389515463408878255e3_f64 * t761 * t9722;
    let t9726 = t718 * t2517;
    let t9729 = 1.0_f64 / t2475 / t723;
    (t9715, t9717, t9720, t9722, t9724, t9726, t9729)
}
