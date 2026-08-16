//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1210/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1210(t126323: f64, t126327: f64, t121809: f64, t27186: f64, t121901: f64, t120045: f64, t120048: f64, t120057: f64, t121846: f64, t121980: f64, t121990: f64, t126319: f64, t126325: f64, t126340: f64, t27312: f64) -> f64 {
    let t127809 = 0.150583822711895824e-3_f64 * t126323;
    let t127811 = 0.1054086758983270768e-1_f64 * t126327;
    let t127814 = t121809 * t27186;
    let t127816 = t121901 * t27186;
    let t127821 = -0.225875734067843736e-2_f64 * t126319 + t127809 - 0.26773803678175077509e-3_f64 * t126325 + t127811 - t120045 - 0.69416347856895220196e-2_f64 * t120048 - t121980 + 0.56468933516960933999e-3_f64 * t126340 + 0.28559868832551176308e-1_f64 * t127814 - 0.50779446784275991476e-1_f64 * t127816 + 0.3427184259906141157e1_f64 * t120057 * t121846 * t27312 + t121990;
    t127821
}
