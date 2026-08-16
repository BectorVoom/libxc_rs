//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1469;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1470;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1471;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1472;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta264(t10069: f64, t4089: f64, t138: f64, t2438: f64, t785: f64, t555: f64, t9990: f64, t1432: f64, t2470: f64, t4107: f64, t1433: f64, t9288: f64, t136: f64, t1419: f64, t2457: f64, t3964: f64, t225: f64, t9646: f64, t1428: f64, t22: f64, t2452: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10070, t10073) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1469(t10069, t4089, t138, t2438, t785);
        let (t10074, t10090, t10098, t10102, t10107, t10109) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1470(t10073, t4089, t555, t9990, t1432, t2470, t4107, t1433, t9288, t136, t1419, t2457, t3964);
        let t10111 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1471(t225, t9646);
        let (t10114, t10115) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1472(t10111, t1428, t22, t2452);
    (t10070, t10073, t10074, t10090, t10098, t10102, t10107, t10109, t10111, t10114, t10115)
}
