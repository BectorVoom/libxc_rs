//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta179 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk883;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk884;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk885;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta179(t2580: f64, t680: f64, t130: f64, t146: f64, t2583: f64, t9275: f64, t2514: f64, t2596: f64, t746: f64, t1340: f64, t2491: f64, t2495: f64, t744: f64, t215: f64, t681: f64, t268: f64, t702: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9310, t9311, t9313, t9314, t9316) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk883(t2580, t680, t130, t146, t2583, t9275);
        let t9318 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk884(t2514, t2596, t746);
        let (t9320, t9321, t9323) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk885(t1340, t9318, t2491, t2514, t2495, t744);
        let (t9325, t9326, t9329) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk886(t1340, t9323, t215, t681, t268, t702);
    (t9310, t9311, t9313, t9314, t9316, t9318, t9320, t9321, t9323, t9325, t9326, t9329)
}
