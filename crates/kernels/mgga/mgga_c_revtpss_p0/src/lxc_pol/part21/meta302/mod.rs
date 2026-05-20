//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1553;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1554;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta302<F: Float>(t10626: F, t10627: F, t775: F, t853: F, t2430: F, t10489: F, t832: F, t10618: F, t227: F, t229: F, t2634: F, t2639: F, t2642: F, t4415: F, t830: F, t833: F, t231: F, t2710: F, t2793: F, t9285: F, t2470: F, t2804: F, t874: F, t875: F, t9288: F, t251: F, t2722: F, t2723: F, t4503: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10628, t10631, t10632, t10635, t10638) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1553::<F>(t10626, t10627, t775, t853, t2430, t10489, t832, t10618, t227, t229, t2634, t2639, t2642, t4415, t830, t833);
        let t10639 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1554::<F>(t10638, t231);
        let (t10645, t10647, t10651, t10652, t10654) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1555::<F>(t2710, t2793, t9285, t2470, t2804, t874, t875, t9288, t251, t2722, t2723, t4503);
    (t10628, t10631, t10632, t10635, t10638, t10639, t10645, t10647, t10651, t10652, t10654)
}
