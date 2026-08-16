//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta226 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk966;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk967;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk968;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk969;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk970;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk971;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk972;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk973;
use chunk8::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk974;
use chunk9::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk975;
use chunk10::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk976;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta226<F: Float>(t225: F, t6041: F, t1579: F, t2770: F, t1559: F, t213: F, t234: F, t2776: F, t2780: F, t2796: F, t2810: F, t2811: F, t4497: F, t4501: F, t4520: F, t4524: F, t4526: F, t5978: F, t6017: F, t6022: F, t820: F, t879: F, t868: F, t1580: F, t2437: F, t2443: F, t2460: F, t2473: F, t257: F, t4323: F, t4326: F, t4474: F, t4478: F, t4482: F, t865: F, t1583: F, t198: F, t207: F, t2393: F, t2403: F, t2411: F, t2621: F, t5927: F, t5943: F, t5945: F, t5947: F, t5948: F, t5962: F, t5966: F, t5970: F, t6001: F, t6004: F, t765: F, t892: F, t5926: F, t2852: F, t5819: F, t2850: F, t128: F, t2857: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6042, t6048) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk966::<F>(t225, t6041, t1579);
        let t6049 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk967::<F>(t2770, t6048);
        let t6071 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk968::<F>(t1559, t213, t234, t2776, t2780, t2796, t2810, t2811, t4497, t4501, t4520, t4524, t4526, t5978, t6017, t6022, t6041, t820, t879);
        let t6072 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk969::<F>(t6071, t868);
        let t6075 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk970::<F>(t1580, t213, t2437, t2443, t2460, t2473, t257, t4323, t4326, t4474, t4478, t4482, t6042, t6049, t6072, t865);
        let t6079 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk971::<F>(t1583);
        let t6083 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk972::<F>(t198, t207, t2393, t2403, t2411, t2621, t5927, t5943, t5945, t5947, t5948, t5962, t5966, t5970, t6001, t6004, t6075, t6079, t765, t892);
        let t6084 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk973::<F>(t5926, t6083);
        let t6092 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk974::<F>(t2852, t5819);
        let (t6093, t6094) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk975::<F>(t2850, t6092, t128);
        let t6096 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk976::<F>(t2857, t5819);
    (t6042, t6048, t6049, t6071, t6072, t6075, t6079, t6084, t6092, t6093, t6094, t6096)
}
