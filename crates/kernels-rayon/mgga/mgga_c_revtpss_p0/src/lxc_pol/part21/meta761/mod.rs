//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta761 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2696;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2697;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2698;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2699;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2700;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2701;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta761(t5778: f64, t9593: f64, t39750: f64, t39756: f64, t39760: f64, t4144: f64, t46992: f64, t46996: f64, t46998: f64, t47003: f64, t48252: f64, t48254: f64, t48256: f64, t5541: f64, t1868: f64, t4135: f64, t13586: f64, t3889: f64, t39799: f64, t4139: f64, t47059: f64, t48265: f64, t48266: f64, t48268: f64, t48270: f64, t48271: f64, t48275: f64, t5536: f64, t5537: f64, t7315: f64, t9628: f64, t1353: f64, t13716: f64, t4140: f64, t47076: f64, t48281: f64, t48283: f64, t48284: f64, t48286: f64, t48288: f64, t48291: f64, t48293: f64, t48295: f64, t566: f64, t1448: f64, t3829: f64, t39989: f64, t47086: f64, t47088: f64, t47092: f64, t47096: f64, t47098: f64, t48305: f64, t48307: f64, t48308: f64, t48311: f64, t5542: f64, t14304: f64, t1450: f64, t47109: f64, t47116: f64, t47118: f64, t47122: f64, t47124: f64, t48315: f64, t48316: f64, t48317: f64, t48318: f64, t48319: f64, t48320: f64, t1907: f64, t47672: f64, t1343: f64, t198: f64, t40079: f64, t47152: f64, t47638: f64, t48328: f64, t48329: f64, t48330: f64, t48332: f64, t48334: f64, t48336: f64, t48421: f64, t9590: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t49579 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2696(t5778, t9593, t39750, t39756, t39760, t4144, t46992, t46996, t46998, t47003, t48252, t48254, t48256, t5541);
        let t49592 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2697(t1868, t4135, t13586, t3889, t39799, t4139, t47059, t48265, t48266, t48268, t48270, t48271, t48275, t5536, t5537, t7315, t9628);
        let t49611 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2698(t1353, t13716, t4139, t4140, t47076, t48281, t48283, t48284, t48286, t48288, t48291, t48293, t48295, t5536, t566);
        let (t49616, t49634) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2699(t1448, t3829, t3889, t39989, t4139, t47086, t47088, t47092, t47096, t47098, t48305, t48307, t48308, t48311, t5542);
        let (t49640, t49647, t49654, t49659) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2700(t1353, t4135, t14304, t1450, t1448, t47109, t47116, t47118, t47122, t47124, t48315, t48316, t48317, t48318, t48319, t48320);
        let t49675 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2701(t1907, t47672, t1343, t1868, t198, t40079, t4139, t47152, t47638, t48328, t48329, t48330, t48332, t48334, t48336, t48421, t5541, t9590);
    (t49579, t49592, t49611, t49616, t49634, t49640, t49647, t49654, t49659, t49675)
}
