//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta265 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1468;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1469;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1470;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1471;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1472;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta265(t125: f64, t3923: f64, t3936: f64, t3938: f64, t3937: f64, t4057: f64, t5673: f64, t1353: f64, t4003: f64, t4056: f64, t2735: f64, t4086: f64, t3994: f64, t808: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9365: f64, t9374: f64, t9376: f64, t9389: f64, t9391: f64, t9394: f64, t9396: f64, t9399: f64, t9405: f64, t9407: f64, t9409: f64, t9412: f64, t9415: f64, t9421: f64, t9423: f64, t9427: f64, t9430: f64, t9546: f64, t9514: f64, t9517: f64, t9521: f64, t9553: f64, t9556: f64, t9560: f64, t9562: f64, t9565: f64, t9567: f64, t9569: f64, t9571: f64, t9574: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9826, t9828, t9832, t9835) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1468(t125, t3923, t3936, t3938, t3937, t4057, t5673, t1353, t4003);
        let (t9837, t9840) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1469(t3936, t9826, t9835, t4003, t4056);
        let (t9842, t9845) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1470(t3937, t5673, t9840, t2735, t4086);
        let (t9846, t9847, t9849) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1471(t3994, t808, t9845, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9365, t9374, t9376, t9389, t9391);
        let t9850 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1472(t9394, t9396, t9399, t9405, t9407, t9409, t9412, t9415, t9421, t9423, t9427, t9430, t9546);
        let t9852 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1473(t9514, t9517, t9521, t9553, t9556, t9560, t9562, t9565, t9567, t9569, t9571, t9574);
    (t9828, t9832, t9835, t9837, t9840, t9842, t9845, t9846, t9847, t9849, t9850, t9852)
}
