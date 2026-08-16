//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta67 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk420;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk421;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk422;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk423;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta67(t1120: f64, t1715: f64, t128: f64, t1119: f64, t422: f64, t1118: f64, t1132: f64, t1139: f64, t1145: f64, t141: f64, t1137: f64, t1144: f64, t1150: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1716, t1717, t1719) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk420(t1120, t1715, t128, t1119);
        let (t1721, t1723) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk421(t1719, t422, t1118, t1717);
        let (t1724, t1727, t1729, t1730, t1732) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk422(t1132, t1723, t1139, t1145, t1715, t141, t1137, t1144, t1717);
        let t1733 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk423(t1150, t1732);
    (t1716, t1717, t1719, t1721, t1723, t1724, t1727, t1729, t1730, t1732, t1733)
}
