//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1034 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3618;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1034<F: Float>(t20273: F, t698: F, t1145: F, t141: F, t68391: F, t3417: F, t68280: F, t68285: F, t1139: F, t68463: F, t2439: F, t6467: F, t6464: F, t68251: F, t6461: F, t68395: F, t58209: F, t58211: F, t58225: F, t68456: F, t68459: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t68567, t68570, t68573, t68576, t68578, t68583) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3618::<F>(t20273, t698, t1145, t141, t68391, t3417, t68280, t68285, t1139, t68463, t2439, t6467);
        let (t68585, t68588, t68590, t68593, t68595) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3619::<F>(t2439, t6464, t1145, t141, t68251, t6461, t3417, t68395, t58209, t58211, t58225, t68456, t68459, t68567, t68570, t68573, t68576, t68578, t68583);
    (t68567, t68570, t68573, t68576, t68578, t68583, t68585, t68588, t68590, t68593, t68595)
}
