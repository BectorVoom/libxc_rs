//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta268 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1198;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1199;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1200;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1201;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1202;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1203;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta268(t1203: f64, t2142: f64, t7637: f64, t2147: f64, t3565: f64, t7635: f64, t1214: f64, t1269: f64, t2148: f64, t3736: f64, t473: f64, t1294: f64, t3140: f64, t487: f64, t1276: f64, t1243: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7639, t7642) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1198(t1203, t2142, t7637, t2147, t3565);
        let t7643 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1199(t7635, t7642);
        let (t7645, t7648, t7651) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1200(t1214, t2142, t7637, t1269, t2148, t7635);
        let t7652 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1201(t3736, t473);
        let (t7654, t7658, t7659) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1202(t1294, t2142, t7652, t3140, t487, t1276, t2148);
        let t7660 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1203(t1243, t2142);
    (t7639, t7642, t7643, t7645, t7648, t7651, t7652, t7654, t7658, t7659, t7660)
}
