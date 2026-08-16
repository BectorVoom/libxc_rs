//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1762;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1763;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta319<F: Float>(t775: F, t853: F, t2710: F, t2793: F, t9285: F, t2470: F, t2804: F, t874: F, t875: F, t9288: F, t251: F, t2722: F, t2723: F, t4503: F, t2782: F, t2760: F, t822: F, t2718: F, t860: F, t243: F, t816: F, t9707: F, t813: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10631, t10645, t10647, t10651, t10652) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1762::<F>(t775, t853, t2710, t2793, t9285, t2470, t2804, t874, t875, t9288, t251, t2722);
        let (t10654, t10655, t10657, t10661, t10673) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1763::<F>(t10652, t2723, t4503, t2782, t2760, t822, t2718, t860, t243, t816, t9707, t813);
    (t10631, t10645, t10647, t10651, t10652, t10654, t10655, t10657, t10661, t10673)
}
