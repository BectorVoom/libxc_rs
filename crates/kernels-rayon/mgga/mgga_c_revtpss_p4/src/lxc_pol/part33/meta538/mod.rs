//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta538 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1899;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1900;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1901;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1902;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1903;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1904;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1905;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta538(t800: f64, t8171: f64, t26865: f64, t4890: f64, t3767: f64, t3782: f64, t1227: f64, t1238: f64, t1266: f64, t26867: f64, t26870: f64, t26877: f64, t29083: f64, t29086: f64, t5335: f64, t5343: f64, t5348: f64, t5354: f64, t5369: f64, t5397: f64, t5402: f64, t7607: f64, t7624: f64, t1252: f64, t1797: f64, t26873: f64, t26880: f64, t29010: f64, t29020: f64, t29023: f64, t29027: f64, t29052: f64, t29079: f64, t5270: f64, t5279: f64, t5287: f64, t5299: f64, t5304: f64, t7618: f64, t2150: f64, t473: f64, t2142: f64, t5245: f64, t7637: f64, t1243: f64, t8190: f64, t1248: f64, t1287: f64, t1811: f64, t3140: f64, t1276: f64, t2148: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t29089 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1899(t800, t8171);
        let t29096 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1900(t26865, t4890);
        let t29097 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1901(t29096, t3767);
        let t29100 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1902(t29096, t3782);
        let t29107 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1903(t1227, t1238, t1266, t26867, t26870, t26877, t29083, t29086, t29089, t29097, t29100, t5335, t5343, t5348, t5354, t5369, t5397, t5402, t7607, t7624);
        let t29109 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1904(t1252, t1797, t26873, t26880, t29010, t29020, t29023, t29027, t29052, t29079, t29107, t5270, t5279, t5287, t5299, t5304, t7618, t7624);
        let (t29111, t29118, t29119, t29122, t29124, t29127) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1905(t2150, t29109, t473, t2142, t5245, t7637, t1243, t8190, t1248, t1287, t1811, t3140);
        let t29129 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1906(t1276, t2148, t29127);
    (t29089, t29096, t29097, t29100, t29109, t29111, t29118, t29119, t29122, t29124, t29129)
}
