//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2150;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2151;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2152;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2153;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta568<F: Float>(t124: F, t22813: F, t800: F, t1883: F, t22079: F, t5673: F, t1872: F, t6816: F, t22046: F, t3936: F, t6869: F, t543: F, t6836: F, t5674: F, t9955: F, t13858: F, t13949: F, t13956: F, t22103: F, t22127: F, t22131: F, t3934: F, t3944: F, t9748: F, t9786: F, t9791: F, t9804: F, t22857: F, t1390: F, t828: F, t22762: F, t22763: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22876, t22877, t22881, t22886, t22890, t22893) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2150::<F>(t124, t22813, t800, t1883, t22079, t5673, t1872, t6816, t22046, t3936, t6869, t543, t6836);
        let (t22895, t22903) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2151::<F>(t22893, t5674, t9955, t13858, t13949, t13956, t22103, t22127, t22131, t22877, t22881, t22886, t22890, t3934, t3944, t9748, t9786, t9791, t9804);
        let t22912 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2152::<F>(t22857, t543);
        let (t22914, t22917) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2153::<F>(t1390, t22912, t828, t22762, t22763, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
    (t22876, t22877, t22881, t22886, t22890, t22893, t22895, t22903, t22912, t22914, t22917)
}
