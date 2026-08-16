//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta87 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk505;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk506;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk507;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk508;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk509;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk510;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta87(t30: f64, t33: f64, t1312: f64, t1502: f64, t1518: f64, t1468: f64, t513: f64, t1711: f64, t516: f64, t162: f64, zeta_threshold: f64, t189: f64, t512: f64, t187: f64, t1344: f64, t1348: f64, t124: f64, t800: f64, t1319: f64, t1322: f64, t1334: f64, t1339: f64, t1342: f64, t225: f64, t679: f64, t704: f64, t1394: f64, t539: f64, t541: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1847, t1856) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk505(t30, t33, t1312, t1502, t1518, t1468, t513, t1711, t516, t162, zeta_threshold);
        let t1857 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk506(t1856, t189);
        let (t1858, t1860, t1868) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk507(t30, t33, t1857, t512, t1856, t187, t1344, t1468, t1348, t1711, zeta_threshold);
        let (t1872, t1873, t1877) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk508(t124, t1868, t800, t1319, t1322, t1334, t1339, t1342, t1858, t1860, t225, t679, t704);
        let (t1879, t1882) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk509(t1394, t1868, t1877, t539, t541);
        let t1883 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk510(t1882, t543);
    (t1847, t1856, t1857, t1858, t1860, t1868, t1872, t1873, t1877, t1879, t1882, t1883)
}
