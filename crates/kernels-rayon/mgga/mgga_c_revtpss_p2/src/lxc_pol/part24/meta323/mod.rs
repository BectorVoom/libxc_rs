//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta323 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1119;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1120;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1121;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1122;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1123;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1124;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta323(t30: f64, t13611: f64, t1468: f64, t6785: f64, t22670: f64, t513: f64, t5549: f64, t5824: f64, t9335: f64, t1711: f64, t6792: f64, zeta_threshold: f64, t33: f64, t516: f64, t5557: f64, t6416: f64, t9350: f64, t162: f64, t189: f64, t512: f64, t1344: f64, t5574: f64, t9605: f64, t1348: f64, t5582: f64, t9617: f64, t1868: f64, t6836: f64, t828: f64, t9942: f64, t1414: f64, t22079: f64, t3936: f64, t6869: f64, t13790: f64, t5673: f64, t1883: f64, t22074: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22768, t22769, t22777, t22778, t22783) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1119(t30, t13611, t1468, t6785, t22670, t513, t5549, t5824, t9335, t1711, t6792, zeta_threshold);
        let t22789 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1120(t33, t22778, t22783, t516, t5557, t6416, t9350, t162, t22777, zeta_threshold);
        let (t22790, t22791, t22799, t22807) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1121(t30, t33, t189, t22789, t512, t1344, t22670, t22769, t5574, t5824, t9605, t1348, t22778, t22783, t5582, t6416, t9617, zeta_threshold);
        let t22809 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1122(t22799, t22807);
        let t22813 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1123(t1868, t6836);
        let (t22815, t22822, t22829, t22833, t22837) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1124(t22813, t828, t9942, t1414, t22809, t22079, t3936, t6869, t13790, t5673, t1883, t22074);
    (t22768, t22783, t22789, t22790, t22791, t22809, t22813, t22815, t22822, t22829, t22833, t22837)
}
