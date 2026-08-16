//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1130;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1131;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1132;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta325(t124: f64, t22813: f64, t800: f64, t1883: f64, t22079: f64, t5673: f64, t1872: f64, t6816: f64, t22046: f64, t3936: f64, t6869: f64, t543: f64, t6836: f64, t5674: f64, t9955: f64, t13858: f64, t13949: f64, t13956: f64, t22103: f64, t22127: f64, t22131: f64, t3934: f64, t3944: f64, t9748: f64, t9786: f64, t9791: f64, t9804: f64, t22857: f64, t1390: f64, t828: f64, t22762: f64, t22763: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9374: f64, t9389: f64, t9391: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22877, t22881, t22886, t22890, t22893) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1130(t124, t22813, t800, t1883, t22079, t5673, t1872, t6816, t22046, t3936, t6869, t543, t6836);
        let (t22895, t22903) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1131(t22893, t5674, t9955, t13858, t13949, t13956, t22103, t22127, t22131, t22877, t22881, t22886, t22890, t3934, t3944, t9748, t9786, t9791, t9804);
        let t22912 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1132(t22857, t543);
        let (t22914, t22917) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1133(t1390, t22912, t828, t22762, t22763, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
    (t22877, t22881, t22886, t22890, t22893, t22895, t22903, t22912, t22914, t22917)
}
