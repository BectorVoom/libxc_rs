//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1907;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1908;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1909;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1910;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta539(t2148: f64, t5412: f64, t1032: f64, t1811: f64, t7642: f64, t1294: f64, t8208: f64, t26969: f64, t1775: f64, t1829: f64, t2149: f64, t2152: f64, t27008: f64, t27011: f64, t27025: f64, t29111: f64, t29119: f64, t29124: f64, t29129: f64, t5246: f64, t7602: f64, t7643: f64, t7645: f64, t7648: f64, t7651: f64, t7654: f64, t7659: f64, t7662: f64, t7666: f64, t8198: f64, t8205: f64, t8217: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t29132, t29135) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1907(t2148, t5412, t1032, t1811);
        let t29136 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1908(t29135, t7642);
        let t29141 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1909(t2148, t29135);
        let (t29149, t29154) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1910(t1294, t8208, t26969, t1775, t1829, t2149, t2152, t27008, t27011, t27025, t29111, t29119, t29124, t29129, t29132, t29136, t29141, t5246, t7602, t7643, t7645, t7648, t7651, t7654, t7659, t7662, t7666, t8198, t8205, t8217);
    (t29132, t29135, t29136, t29141, t29149, t29154)
}
