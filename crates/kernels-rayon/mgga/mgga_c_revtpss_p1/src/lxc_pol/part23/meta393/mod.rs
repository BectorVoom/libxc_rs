//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta393 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1745;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1746;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1747;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1748;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta393(t1263: f64, t1794: f64, t372: f64, t11262: f64, t1796: f64, t1247: f64, t12915: f64, t247: f64, t5230: f64, t5384: f64, t1770: f64, t3140: f64, t3609: f64, t12772: f64, t5406: f64, t3625: f64, t1802: f64, t474: f64, t3089: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17352, t17353) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1745(t1263, t1794, t372);
        let (t17361, t17362, t17373, t17375, t17376, t17377, t17384) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1746(t11262, t1796, t1247, t12915, t247, t5230, t5384, t1770, t3140, t3609, t12772, t5406);
        let (t17386, t17394) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1747(t17384, t3625, t1802, t474);
        let t17395 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1748(t17394, t3089);
    (t17352, t17353, t17361, t17362, t17373, t17375, t17376, t17377, t17384, t17386, t17394, t17395)
}
