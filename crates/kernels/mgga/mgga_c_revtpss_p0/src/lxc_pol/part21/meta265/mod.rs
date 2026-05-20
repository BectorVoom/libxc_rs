//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta265 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1468;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1469;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1470;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1471;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1472;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta265<F: Float>(t125: F, t3923: F, t3936: F, t3938: F, t3937: F, t4057: F, t5673: F, t1353: F, t4003: F, t4056: F, t2735: F, t4086: F, t3994: F, t808: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9365: F, t9374: F, t9376: F, t9389: F, t9391: F, t9394: F, t9396: F, t9399: F, t9405: F, t9407: F, t9409: F, t9412: F, t9415: F, t9421: F, t9423: F, t9427: F, t9430: F, t9546: F, t9514: F, t9517: F, t9521: F, t9553: F, t9556: F, t9560: F, t9562: F, t9565: F, t9567: F, t9569: F, t9571: F, t9574: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9826, t9828, t9832, t9835) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1468::<F>(t125, t3923, t3936, t3938, t3937, t4057, t5673, t1353, t4003);
        let (t9837, t9840) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1469::<F>(t3936, t9826, t9835, t4003, t4056);
        let (t9842, t9845) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1470::<F>(t3937, t5673, t9840, t2735, t4086);
        let (t9846, t9847, t9849) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1471::<F>(t3994, t808, t9845, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9365, t9374, t9376, t9389, t9391);
        let t9850 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1472::<F>(t9394, t9396, t9399, t9405, t9407, t9409, t9412, t9415, t9421, t9423, t9427, t9430, t9546);
        let t9852 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1473::<F>(t9514, t9517, t9521, t9553, t9556, t9560, t9562, t9565, t9567, t9569, t9571, t9574);
    (t9828, t9832, t9835, t9837, t9840, t9842, t9845, t9846, t9847, t9849, t9850, t9852)
}
