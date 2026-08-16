//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1301/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1301(t13109: f64, t13113: f64, t5398: f64, t751: f64, t707: f64, t13133: f64, t1462: f64, t2427: f64, t5597: f64, t9922: f64, t13124: f64, t5522: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16699 = 0.48830526149350786811e-3_f64 * t13109;
    let t16700 = 0.11696447245269292414e1_f64 * t13113;
    let t16701 = t751 * t5398;
    let t16702 = t707 * t16701;
    let t16703 = 4.0_f64 * t16702;
    let t16705 = 8.0_f64 * t13133 * t1462;
    let t16707 = 4.0_f64 * t2427 * t5597;
    let t16708 = 0.5848223622634646207e0_f64 * t9922;
    let t16709 = 0.21687162600603479684e-1_f64 * t13124;
    let t16710 = t5522 * t67;
    (t16699, t16700, t16703, t16705, t16707, t16708, t16709, t16710)
}
