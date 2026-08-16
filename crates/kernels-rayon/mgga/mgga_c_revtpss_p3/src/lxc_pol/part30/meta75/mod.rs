//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta75 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk488;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk489;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk490;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk491;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk492;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta75(t1469: f64, t36: f64, t70: f64, t48: f64, t51: f64, t53: f64, rho1: f64, sigma2: f64, t60: f64, t44: f64, t56: f64, t61: f64, t626: f64, t38: f64, t633: f64, t637: f64, t77: f64, t71: f64, t85: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1470 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk488(t1469, t36);
        let (t1471, t1474, t1479) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk489(t1470, t70, t1469, t48, t51, t53, rho1);
        let t1480 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk490(t1479, sigma2);
        let (t1486, t1487, t1490, t1491, t1493) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk491(t1469, t60, t1474, t1480, t44, t56, t61, t626, t38, t633, t637);
        let (t1494, t1497) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk492(t1493, t77, t1471, t1487, t71, t85);
    (t1470, t1471, t1474, t1479, t1480, t1486, t1487, t1490, t1491, t1493, t1494, t1497)
}
