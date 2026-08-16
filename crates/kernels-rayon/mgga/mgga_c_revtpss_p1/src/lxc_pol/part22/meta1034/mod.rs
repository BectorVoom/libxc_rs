//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1034 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3618;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1034(t20273: f64, t698: f64, t1145: f64, t141: f64, t68391: f64, t3417: f64, t68280: f64, t68285: f64, t1139: f64, t68463: f64, t2439: f64, t6467: f64, t6464: f64, t68251: f64, t6461: f64, t68395: f64, t58209: f64, t58211: f64, t58225: f64, t68456: f64, t68459: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68567, t68570, t68573, t68576, t68578, t68583) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3618(t20273, t698, t1145, t141, t68391, t3417, t68280, t68285, t1139, t68463, t2439, t6467);
        let (t68585, t68588, t68590, t68593, t68595) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3619(t2439, t6464, t1145, t141, t68251, t6461, t3417, t68395, t58209, t58211, t58225, t68456, t68459, t68567, t68570, t68573, t68576, t68578, t68583);
    (t68567, t68570, t68573, t68576, t68578, t68583, t68585, t68588, t68590, t68593, t68595)
}
