//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta215 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1266;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1267;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1268;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1269;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1270;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1271;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1272;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta215(t6141: f64, t935: f64, t915: f64, t2926: f64, t6109: f64, t2924: f64, t2930: f64, t4571: f64, t6094: f64, t6098: f64, t6102: f64, t1621: f64, t954: f64, t2950: f64, t2957: f64, t4620: f64, t6114: f64, t6121: f64, t6127: f64, t6129: f64, t6133: f64, t6136: f64, t6139: f64, t2970: f64, t2974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6142 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1266(t6141, t935);
        let (t6144, t6145) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1267(t6142, t915, t2926, t6109);
        let (t6147, t6152, t6157, t6158) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1268(t2924, t6145, t2930, t4571, t6094, t6098, t6102, t1621, t954);
        let t6173 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1269(t2950, t2957, t4571, t4620, t6094, t6098, t6102, t6114, t6121, t6127, t6129, t6133, t6136, t6139);
        let t6174 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1270(t6173, t954);
        let t6177 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1271(t2970, t6157);
        let t6184 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1272(t2974, t4571, t6094, t6098, t6102);
    (t6142, t6144, t6145, t6147, t6152, t6157, t6158, t6173, t6174, t6177, t6184)
}
