//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 968/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk968(t550: f64, t9721: f64, t268: f64, t9718: f64, t64: f64, t8779: f64, t159: f64, t535: f64, t2236: f64, t65: f64, t235: f64, t1389: f64, t3964: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9722 = t9721 * t550;
    let t9723 = t9722 * t268;
    let t9725 = 0.20082057720118594944e-6_f64 * t9718 * t9723;
    let t9726 = t64 * t8779;
    let t9727 = t9726 * t159;
    let t9729 = 455.0_f64 / 1296.0_f64 * t9727 * t535;
    let t9731 = 1.0_f64 / t65 / t2236;
    let t9732 = t235 * t9731;
    let t9735 = 0.81322168495418382223e-4_f64 * t3964 * t9732 * t1389;
    (t9725, t9727, t9729, t9731, t9732, t9735)
}
