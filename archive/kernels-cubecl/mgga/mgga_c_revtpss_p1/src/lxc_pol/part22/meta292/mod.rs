//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1709;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1710;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta292<F: Float>(t3981: F, t9765: F, t3923: F, t550: F, t543: F, t3992: F, t2661: F, t212: F, t225: F, t596: F, t816: F, t3995: F, t1408: F, t2681: F, t820: F, t1416: F, t124: F, t2237: F, t800: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9766, t9768, t9769, t9770, t9771, t9775) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1709::<F>(t3981, t9765, t3923, t550, t543, t3992, t2661, t212, t225, t596, t816);
        let (t9776, t9779) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1710::<F>(t3995, t9775, t1408, t2681, t820);
        let (t9780, t9784) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1711::<F>(t1416, t9779, t124, t212, t2237, t800);
    (t9766, t9768, t9769, t9770, t9771, t9775, t9776, t9779, t9780, t9784)
}
