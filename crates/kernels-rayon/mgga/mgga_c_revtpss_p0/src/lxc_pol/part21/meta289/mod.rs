//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1527;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1528;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1529;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1530;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1531;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta289(t10355: f64, t10356: f64, t2275: f64, t606: f64, t2258: f64, t10326: f64, t48: f64, t58: f64, t59: f64, t2282: f64, t60: f64, t10199: f64, t10345: f64, t2270: f64, t2276: f64, t2279: f64, t44: f64, t49: f64, t56: f64, t614: f64, t617: f64, t38: f64, t2851: f64, t78: f64, t2299: f64, t3361: f64, t81: f64, t2306: f64, t633: f64, t637: f64, t77: f64, t10317: f64, t10318: f64, t10321: f64, t10328: f64, t10331: f64, t10336: f64, t2252: f64, t2260: f64, t2263: f64, t2292: f64, t2312: f64, t608: f64, t628: f64, t641: f64, t71: f64, t85: f64, t5: f64, t10296: f64, t10298: f64, t10301: f64, t10309: f64, t10310: f64, t10313: f64, t2242: f64, t2247: f64, t2248: f64, t2315: f64, t603: f64, t644: f64, t91: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10357, t10361, t10364, t10368, t10369, t10373, t10376, t10379) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1527(t10355, t10356, t2275, t606, t2258, t10326, t48, t58, t59, t2282, t60, t10199);
        let t10380 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1528(t10345, t10357, t10361, t10364, t10369, t10373, t10376, t10379, t2270, t2276, t2279, t44, t49, t56, t614, t617);
        let (t10381, t10389, t10398, t10406) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1529(t10380, t38, t2851, t78, t2299, t606, t3361, t81, t2306, t10326, t10356, t2258, t633, t637);
        let (t10407, t10410) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1530(t10406, t77, t10317, t10318, t10321, t10328, t10331, t10336, t10381, t2252, t2260, t2263, t2292, t2312, t608, t628, t641, t71, t85);
        let t10414 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1531(t5, t10296, t10298, t10301, t10309, t10310, t10313, t10410, t2242, t2247, t2248, t2315, t603, t644, t91);
    (t10368, t10369, t10373, t10376, t10379, t10380, t10381, t10389, t10398, t10407, t10410, t10414)
}
