//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1553;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1554;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta302(t10626: f64, t10627: f64, t775: f64, t853: f64, t2430: f64, t10489: f64, t832: f64, t10618: f64, t227: f64, t229: f64, t2634: f64, t2639: f64, t2642: f64, t4415: f64, t830: f64, t833: f64, t231: f64, t2710: f64, t2793: f64, t9285: f64, t2470: f64, t2804: f64, t874: f64, t875: f64, t9288: f64, t251: f64, t2722: f64, t2723: f64, t4503: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10628, t10631, t10632, t10635, t10638) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1553(t10626, t10627, t775, t853, t2430, t10489, t832, t10618, t227, t229, t2634, t2639, t2642, t4415, t830, t833);
        let t10639 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1554(t10638, t231);
        let (t10645, t10647, t10651, t10652, t10654) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1555(t2710, t2793, t9285, t2470, t2804, t874, t875, t9288, t251, t2722, t2723, t4503);
    (t10628, t10631, t10632, t10635, t10638, t10639, t10645, t10647, t10651, t10652, t10654)
}
