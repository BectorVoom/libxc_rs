//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1308;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1309;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1310;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta394(t258: f64, t39552: f64, t2454: f64, t2455: f64, t39494: f64, t14545: f64, t251: f64, t786: f64, t2710: f64, t2793: f64, t211: f64, t9644: f64, t209: f64, t234: f64, t268: f64, t8779: f64, t39497: f64, t874: f64, t875: f64, t10529: f64, t2453: f64, t253: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39554, t39557, t39598, t39633, t39643) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1308(t258, t39552, t2454, t2455, t39494, t14545, t251, t786, t2710, t2793, t211, t9644);
        let t39644 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1309(t209, t39643);
        let (t39649, t39652, t39680, t39697) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1310(t234, t251, t268, t39644, t8779, t39497, t874, t875, t10529, t2453, t253, t39552);
    (t39554, t39557, t39598, t39633, t39644, t39649, t39652, t39680, t39697)
}
