//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk446;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk447;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk448;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk449;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk450;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta74(t1469: f64, t36: f64, t70: f64, t48: f64, t51: f64, t53: f64, rho1: f64, sigma2: f64, t60: f64, t44: f64, t56: f64, t61: f64, t626: f64, t38: f64, t633: f64, t637: f64, t77: f64, t71: f64, t85: f64, t5: f64, t1466: f64, t603: f64, t91: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1470, t1471, t1474, t1477, t1479, t1480) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk446(t1469, t36, t70, t48, t51, t53, rho1, sigma2);
        let (t1483, t1486) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk447(t1469, t60, t1474, t1480, t44, t56, t61, t626);
        let (t1487, t1494) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk448(t1486, t38, t1469, t633, t637, t77);
        let t1497 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk449(t1471, t1487, t1494, t71, t85);
        let t1501 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk450(t5, t1466, t1497, t603, t91);
    (t1470, t1471, t1474, t1477, t1479, t1480, t1483, t1486, t1487, t1494, t1497, t1501)
}
