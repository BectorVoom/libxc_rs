//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta761 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2696;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2697;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2698;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2699;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2700;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2701;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta761<F: Float>(t5778: F, t9593: F, t39750: F, t39756: F, t39760: F, t4144: F, t46992: F, t46996: F, t46998: F, t47003: F, t48252: F, t48254: F, t48256: F, t5541: F, t1868: F, t4135: F, t13586: F, t3889: F, t39799: F, t4139: F, t47059: F, t48265: F, t48266: F, t48268: F, t48270: F, t48271: F, t48275: F, t5536: F, t5537: F, t7315: F, t9628: F, t1353: F, t13716: F, t4140: F, t47076: F, t48281: F, t48283: F, t48284: F, t48286: F, t48288: F, t48291: F, t48293: F, t48295: F, t566: F, t1448: F, t3829: F, t39989: F, t47086: F, t47088: F, t47092: F, t47096: F, t47098: F, t48305: F, t48307: F, t48308: F, t48311: F, t5542: F, t14304: F, t1450: F, t47109: F, t47116: F, t47118: F, t47122: F, t47124: F, t48315: F, t48316: F, t48317: F, t48318: F, t48319: F, t48320: F, t1907: F, t47672: F, t1343: F, t198: F, t40079: F, t47152: F, t47638: F, t48328: F, t48329: F, t48330: F, t48332: F, t48334: F, t48336: F, t48421: F, t9590: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t49579 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2696::<F>(t5778, t9593, t39750, t39756, t39760, t4144, t46992, t46996, t46998, t47003, t48252, t48254, t48256, t5541);
        let t49592 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2697::<F>(t1868, t4135, t13586, t3889, t39799, t4139, t47059, t48265, t48266, t48268, t48270, t48271, t48275, t5536, t5537, t7315, t9628);
        let t49611 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2698::<F>(t1353, t13716, t4139, t4140, t47076, t48281, t48283, t48284, t48286, t48288, t48291, t48293, t48295, t5536, t566);
        let (t49616, t49634) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2699::<F>(t1448, t3829, t3889, t39989, t4139, t47086, t47088, t47092, t47096, t47098, t48305, t48307, t48308, t48311, t5542);
        let (t49640, t49647, t49654, t49659) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2700::<F>(t1353, t4135, t14304, t1450, t1448, t47109, t47116, t47118, t47122, t47124, t48315, t48316, t48317, t48318, t48319, t48320);
        let t49675 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2701::<F>(t1907, t47672, t1343, t1868, t198, t40079, t4139, t47152, t47638, t48328, t48329, t48330, t48332, t48334, t48336, t48421, t5541, t9590);
    (t49579, t49592, t49611, t49616, t49634, t49640, t49647, t49654, t49659, t49675)
}
