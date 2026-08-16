//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2431/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2431(t11585: f64, t945: f64, t2935: f64, t2967: f64, t11509: f64, t3006: f64, t11501: f64, t3014: f64, t2866: f64, t2873: f64, t11298: f64, t910: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41794 = t11585 * t945;
    let t41799 = t2935 * t2967;
    let t41813 = t3006 * t11509;
    let t41832 = t11501 * t3014;
    let t41880 = t2866 * t2873;
    let t41883 = t910 * t11298;
    (t41794, t41799, t41813, t41832, t41880, t41883)
}
