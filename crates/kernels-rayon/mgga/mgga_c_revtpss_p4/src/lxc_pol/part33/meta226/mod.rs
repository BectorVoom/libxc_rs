//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta226 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1027;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1028;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1029;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1030;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1031;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1032;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1033;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1034;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1035;
use chunk9::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1036;
use chunk10::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1037;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta226(t225: f64, t6041: f64, t1579: f64, t2770: f64, t1559: f64, t213: f64, t234: f64, t2776: f64, t2780: f64, t2796: f64, t2810: f64, t2811: f64, t4497: f64, t4501: f64, t4520: f64, t4524: f64, t4526: f64, t5978: f64, t6017: f64, t6022: f64, t820: f64, t879: f64, t868: f64, t1580: f64, t2437: f64, t2443: f64, t2460: f64, t2473: f64, t257: f64, t4323: f64, t4326: f64, t4474: f64, t4478: f64, t4482: f64, t865: f64, t1583: f64, t198: f64, t207: f64, t2393: f64, t2403: f64, t2411: f64, t2621: f64, t5927: f64, t5943: f64, t5945: f64, t5947: f64, t5948: f64, t5962: f64, t5966: f64, t5970: f64, t6001: f64, t6004: f64, t765: f64, t892: f64, t5926: f64, t2852: f64, t5819: f64, t2850: f64, t128: f64, t2857: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6042, t6048) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1027(t225, t6041, t1579);
        let t6049 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1028(t2770, t6048);
        let t6071 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1029(t1559, t213, t234, t2776, t2780, t2796, t2810, t2811, t4497, t4501, t4520, t4524, t4526, t5978, t6017, t6022, t6041, t820, t879);
        let t6072 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1030(t6071, t868);
        let t6075 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1031(t1580, t213, t2437, t2443, t2460, t2473, t257, t4323, t4326, t4474, t4478, t4482, t6042, t6049, t6072, t865);
        let t6079 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1032(t1583);
        let t6083 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1033(t198, t207, t2393, t2403, t2411, t2621, t5927, t5943, t5945, t5947, t5948, t5962, t5966, t5970, t6001, t6004, t6075, t6079, t765, t892);
        let t6084 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1034(t5926, t6083);
        let t6092 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1035(t2852, t5819);
        let (t6093, t6094) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1036(t2850, t6092, t128);
        let t6096 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1037(t2857, t5819);
    (t6042, t6048, t6049, t6071, t6072, t6075, t6079, t6084, t6092, t6093, t6094, t6096)
}
