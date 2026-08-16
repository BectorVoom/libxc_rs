//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2942/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2942(t23663: f64, t914: f64, t936: f64, t23798: f64, t945: f64, t23811: f64, t964: f64, t41361: f64, t41549: f64, t51978: f64, t52774: f64, t63276: f64, t63278: f64, t77499: f64, t77503: f64, t77505: f64, t77507: f64, t77509: f64, t77515: f64, t77518: f64, t77521: f64, t77527: f64, t77531: f64, t77535: f64, t77539: f64, t77543: f64, t77547: f64) -> (f64, f64, f64, f64) {
    let t78097 = t23663 * t914;
    let t78099 = 1.0_f64 * t78097 * t936;
    let t78108 = t23798 * t945;
    let t78111 = t23811 * t964;
    let t78132 = 0.65956790123456790123e-2_f64 * t77499 - 0.17808333333333333333e-1_f64 * t77503 + 0.5936111111111111111e-2_f64 * t77505 - 0.23744444444444444444e-1_f64 * t77507 + 0.35616666666666666667e-1_f64 * t77509 - 0.35616666666666666666e-1_f64 * t63276 + 0.11872222222222222222e-1_f64 * t63278 + t41549 + 0.21369999999999999999e0_f64 * t77515 - 0.5936111111111111111e-1_f64 * t77518 - 0.32055e0_f64 * t77521 - t52774 + 0.55403703703703703703e-1_f64 * t51978 + 0.18467901234567901234e-1_f64 * t41361 - 0.35616666666666666666e-1_f64 * t77527 - 0.35616666666666666666e-1_f64 * t77531 + 0.4274e0_f64 * t77535 - 0.32055e0_f64 * t77539 + 0.10685e0_f64 * t77543 + 0.10685e0_f64 * t77547;
    (t78099, t78108, t78111, t78132)
}
