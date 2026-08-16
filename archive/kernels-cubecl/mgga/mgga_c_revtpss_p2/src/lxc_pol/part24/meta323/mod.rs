//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta323 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1119;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1120;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1121;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1122;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1123;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1124;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta323<F: Float>(t30: F, t13611: F, t1468: F, t6785: F, t22670: F, t513: F, t5549: F, t5824: F, t9335: F, t1711: F, t6792: F, zeta_threshold: F, t33: F, t516: F, t5557: F, t6416: F, t9350: F, t162: F, t189: F, t512: F, t1344: F, t5574: F, t9605: F, t1348: F, t5582: F, t9617: F, t1868: F, t6836: F, t828: F, t9942: F, t1414: F, t22079: F, t3936: F, t6869: F, t13790: F, t5673: F, t1883: F, t22074: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22768, t22769, t22777, t22778, t22783) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1119::<F>(t30, t13611, t1468, t6785, t22670, t513, t5549, t5824, t9335, t1711, t6792, zeta_threshold);
        let t22789 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1120::<F>(t33, t22778, t22783, t516, t5557, t6416, t9350, t162, t22777, zeta_threshold);
        let (t22790, t22791, t22799, t22807) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1121::<F>(t30, t33, t189, t22789, t512, t1344, t22670, t22769, t5574, t5824, t9605, t1348, t22778, t22783, t5582, t6416, t9617, zeta_threshold);
        let t22809 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1122::<F>(t22799, t22807);
        let t22813 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1123::<F>(t1868, t6836);
        let (t22815, t22822, t22829, t22833, t22837) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1124::<F>(t22813, t828, t9942, t1414, t22809, t22079, t3936, t6869, t13790, t5673, t1883, t22074);
    (t22768, t22783, t22789, t22790, t22791, t22809, t22813, t22815, t22822, t22829, t22833, t22837)
}
