//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 198/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk198(t48: f64, t606: f64, t60: f64, t579: f64, t66: f64, t64: f64) -> (f64, f64, f64, f64) {
    let t617 = t48 * t606;
    let t620 = t60 * t606;
    let t624 = 1.0_f64 / t66 / t579;
    let t625 = t64 * t624;
    (t617, t620, t624, t625)
}
