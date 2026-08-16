//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1715;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1716;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta462(t25933: f64, t26304: f64, t26292: f64, t7289: f64, t25969: f64, t25975: f64, t26002: f64, t26010: f64, t26012: f64, t26021: f64, t26005: f64, t26007: f64, t26015: f64, t26018: f64, t26025: f64, t26029: f64, t26031: f64, t25973: f64, t25979: f64, t25984: f64, t25988: f64, t25990: f64, t25992: f64, t25994: f64, t25998: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26305, t26309, t26310, t26312, t26321, t26324, t26325, t26328, t26332) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1715(t25933, t26304, t26292, t7289, t25969, t25975, t26002, t26010, t26012, t26021, t26005, t26007, t26015, t26018, t26025, t26029, t26031);
        let t26333 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1716(t25973, t25979, t25984, t25988, t25990, t25992, t25994, t25998, t26310, t26312, t26332);
    (t26305, t26309, t26310, t26312, t26321, t26324, t26325, t26328, t26333)
}
