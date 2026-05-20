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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1899;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1900;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1901;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1902;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1903;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1904;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1905;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta538<F: Float>(t800: F, t8171: F, t26865: F, t4890: F, t3767: F, t3782: F, t1227: F, t1238: F, t1266: F, t26867: F, t26870: F, t26877: F, t29083: F, t29086: F, t5335: F, t5343: F, t5348: F, t5354: F, t5369: F, t5397: F, t5402: F, t7607: F, t7624: F, t1252: F, t1797: F, t26873: F, t26880: F, t29010: F, t29020: F, t29023: F, t29027: F, t29052: F, t29079: F, t5270: F, t5279: F, t5287: F, t5299: F, t5304: F, t7618: F, t2150: F, t473: F, t2142: F, t5245: F, t7637: F, t1243: F, t8190: F, t1248: F, t1287: F, t1811: F, t3140: F, t1276: F, t2148: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t29089 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1899::<F>(t800, t8171);
        let t29096 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1900::<F>(t26865, t4890);
        let t29097 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1901::<F>(t29096, t3767);
        let t29100 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1902::<F>(t29096, t3782);
        let t29107 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1903::<F>(t1227, t1238, t1266, t26867, t26870, t26877, t29083, t29086, t29089, t29097, t29100, t5335, t5343, t5348, t5354, t5369, t5397, t5402, t7607, t7624);
        let t29109 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1904::<F>(t1252, t1797, t26873, t26880, t29010, t29020, t29023, t29027, t29052, t29079, t29107, t5270, t5279, t5287, t5299, t5304, t7618, t7624);
        let (t29111, t29118, t29119, t29122, t29124, t29127) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1905::<F>(t2150, t29109, t473, t2142, t5245, t7637, t1243, t8190, t1248, t1287, t1811, t3140);
        let t29129 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1906::<F>(t1276, t2148, t29127);
    (t29089, t29096, t29097, t29100, t29109, t29111, t29118, t29119, t29122, t29124, t29129)
}
