//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta638 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2341;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2342;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2343;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta638(t211: f64, t9644: f64, t209: f64, t234: f64, t251: f64, t268: f64, t8779: f64, t39497: f64, t874: f64, t875: f64, t10535: f64, t231: f64, t281: f64, t624: f64, t836: f64, t10529: f64, t2453: f64, t253: f64, t39552: f64, t2783: f64, t9646: f64, t22: f64, t837: f64, t10111: f64, t2789: f64, t588: f64, t870: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t39644 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2341(t211, t9644, t209);
        let (t39649, t39652, t39673) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2342(t234, t251, t268, t39644, t8779, t39497, t874, t875, t10535, t231, t281, t624, t836);
        let (t39680, t39697, t39698, t39701, t39719, t39723) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2343(t10529, t2453, t253, t39552, t2783, t9646, t22, t251, t837, t10111, t2789, t588, t870);
    (t39644, t39649, t39652, t39673, t39680, t39697, t39698, t39701, t39719, t39723)
}
