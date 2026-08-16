//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1011/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1011(t547: f64, t9646: f64, t2236: f64, t66: f64, t240: f64, t550: f64, t268: f64, t64: f64, t8779: f64, t159: f64, t535: f64, t65: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9718 = t9646 * t547;
    let t9720 = 1.0_f64 / t66 / t2236;
    let t9721 = t9720 * t240;
    let t9722 = t9721 * t550;
    let t9723 = t9722 * t268;
    let t9725 = 0.20082057720118594944e-6_f64 * t9718 * t9723;
    let t9726 = t64 * t8779;
    let t9727 = t9726 * t159;
    let t9729 = 455.0_f64 / 1296.0_f64 * t9727 * t535;
    let t9731 = 1.0_f64 / t65 / t2236;
    (t9720, t9721, t9725, t9727, t9729, t9731)
}
