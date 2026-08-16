//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2327;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2328;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta532(t1263: f64, t1794: f64, t372: f64, t12712: f64, t3629: f64, t17301: f64, t17304: f64, t17308: f64, t17311: f64, t17333: f64, t17337: f64, t17339: f64, t17340: f64, t17342: f64, t17344: f64, t17347: f64, t17351: f64, t3674: f64, t484: f64, t11262: f64, t1796: f64, t1247: f64, t1264: f64, t16746: f64, t247: f64, t12915: f64, t5230: f64, t5384: f64, t1770: f64, t3140: f64, t3609: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17352, t17353) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2327(t1263, t1794, t372);
        let (t17354, t17355, t17358) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2328(t12712, t3629, t17353, t17301, t17304, t17308, t17311, t17333, t17337, t17339, t17340, t17342, t17344, t17347, t17351, t3674, t484);
        let (t17361, t17362, t17369, t17373, t17375, t17376, t17377) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2329(t11262, t1796, t1247, t1264, t16746, t247, t12915, t5230, t5384, t1770, t3140, t3609);
    (t17352, t17353, t17354, t17355, t17358, t17361, t17362, t17369, t17373, t17375, t17376, t17377)
}
