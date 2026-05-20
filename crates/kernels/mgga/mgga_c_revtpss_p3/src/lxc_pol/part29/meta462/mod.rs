//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1715;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1716;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta462<F: Float>(t25933: F, t26304: F, t26292: F, t7289: F, t25969: F, t25975: F, t26002: F, t26010: F, t26012: F, t26021: F, t26005: F, t26007: F, t26015: F, t26018: F, t26025: F, t26029: F, t26031: F, t25973: F, t25979: F, t25984: F, t25988: F, t25990: F, t25992: F, t25994: F, t25998: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t26305, t26309, t26310, t26312, t26321, t26324, t26325, t26328, t26332) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1715::<F>(t25933, t26304, t26292, t7289, t25969, t25975, t26002, t26010, t26012, t26021, t26005, t26007, t26015, t26018, t26025, t26029, t26031);
        let t26333 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1716::<F>(t25973, t25979, t25984, t25988, t25990, t25992, t25994, t25998, t26310, t26312, t26332);
    (t26305, t26309, t26310, t26312, t26321, t26324, t26325, t26328, t26333)
}
