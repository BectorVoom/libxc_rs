//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta646 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2431;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta646(t11585: f64, t945: f64, t2935: f64, t2967: f64, t11509: f64, t3006: f64, t11501: f64, t3014: f64, t2866: f64, t2873: f64, t11298: f64, t910: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t41794, t41799, t41813, t41832, t41880, t41883) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2431(t11585, t945, t2935, t2967, t11509, t3006, t11501, t3014, t2866, t2873, t11298, t910);
    (t41794, t41799, t41813, t41832, t41880, t41883)
}
