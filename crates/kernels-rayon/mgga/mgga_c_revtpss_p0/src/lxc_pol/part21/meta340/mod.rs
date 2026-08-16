//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1660;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1661;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1662;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1663;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1664;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1665;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1666;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1667;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta340(t11465: f64, t315: f64, t2988: f64, t972: f64, t3014: f64, t11132: f64, t11337: f64, t11158: f64, t11162: f64, t11167: f64, t11316: f64, t11319: f64, t11322: f64, t11326: f64, t11329: f64, t11332: f64, t11339: f64, t11343: f64, t11346: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11171: f64, t11356: f64, t11359: f64, t11366: f64, t11368: f64, t11370: f64, t11373: f64, t11376: f64, t973: f64, t3010: f64, t963: f64, t3013: f64, t323: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t11466 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1660(t11465, t315);
        let t11467 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1661(t2988, t972);
        let (t11468, t11479, t11480, t11485) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1662(t11467, t3014, t11132, t11337, t11158, t11162, t11167, t11316, t11319, t11322, t11326, t11329, t11332, t11339, t11343, t11346);
        let t11500 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1663(t11134, t11136, t11138, t11140, t11147, t11153, t11171, t11356, t11359, t11366, t11368, t11370, t11373, t11376);
        let t11501 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1664(t11485, t11500);
        let (t11502, t11506) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1665(t11501, t973, t3010, t963);
        let t11507 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1666(t11506, t315);
        let t11509 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1667(t3013, t323);
    (t11466, t11467, t11468, t11479, t11480, t11501, t11502, t11506, t11507, t11509)
}
