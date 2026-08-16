//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1201;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1202;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1203;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1204;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta272(t1203: f64, t2142: f64, t7637: f64, t2147: f64, t3565: f64, t7635: f64, t1214: f64, t1269: f64, t2148: f64, t3736: f64, t473: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7638, t7639, t7642) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1201(t1203, t2142, t7637, t2147, t3565);
        let t7643 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1202(t7635, t7642);
        let (t7644, t7645, t7648) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1203(t1214, t2142, t7637, t1269, t2148);
        let t7651 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1204(t2148, t7635);
        let t7652 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1205(t3736, t473);
    (t7638, t7639, t7642, t7643, t7644, t7645, t7648, t7651, t7652)
}
