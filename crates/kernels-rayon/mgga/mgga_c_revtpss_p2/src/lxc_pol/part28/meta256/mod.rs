//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta256 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1138;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1139;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1140;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1141;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1142;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1143;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1144;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta256(t624: f64, t49: f64, t606: f64, t613: f64, t6968: f64, t72: f64, t1927: f64, t640: f64, t76: f64, t1926: f64, t5: f64, t1923: f64, t1928: f64, t6954: f64, t6958: f64, t6960: f64, t6963: f64, t117: f64, t116: f64, t1931: f64, t1937: f64, t2322: f64, t4254: f64, t1310: f64, t1936: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6971, t6972, t6973) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1138(t624, t49, t606, t613, t6968, t72);
        let t6974 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1139(t1927, t6973);
        let t6977 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1140(t640, t76);
        let t6978 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1141(t1926, t6977);
        let (t6982, t6983) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1142(t5, t1923, t1928, t6954, t6958, t6960, t6963, t6974, t6978, t117);
        let t6985 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1143(t116, t1931);
        let (t6990, t6992, t6993) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1144(t1937, t2322, t4254, t1310, t1936);
    (t6971, t6972, t6973, t6974, t6977, t6978, t6982, t6983, t6985, t6990, t6992, t6993)
}
