//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta211 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk989;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk990;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk991;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta211(t10380: f64, t38: f64, t2851: f64, t78: f64, t2299: f64, t606: f64, t3361: f64, t81: f64, t2306: f64, t10326: f64, t10356: f64, t2258: f64, t633: f64, t637: f64, t77: f64, t10317: f64, t10318: f64, t10321: f64, t10328: f64, t10331: f64, t10336: f64, t2252: f64, t2260: f64, t2263: f64, t2292: f64, t2312: f64, t608: f64, t628: f64, t641: f64, t71: f64, t85: f64, t5: f64, t10296: f64, t10298: f64, t10301: f64, t10309: f64, t10310: f64, t10313: f64, t2242: f64, t2247: f64, t2248: f64, t2315: f64, t603: f64, t644: f64, t91: f64, t117: f64, t116: f64, t2319: f64, t10194: f64, t10259: f64, t1312: f64, t2322: f64, t2371: f64, t5523: f64, t670: f64, t2389: f64, t705: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10381, t10389, t10392, t10398, t10401, t10406) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk989(t10380, t38, t2851, t78, t2299, t606, t3361, t81, t2306, t10326, t10356, t2258, t633, t637);
        let (t10407, t10410) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk990(t10406, t77, t10317, t10318, t10321, t10328, t10331, t10336, t10381, t2252, t2260, t2263, t2292, t2312, t608, t628, t641, t71, t85);
        let t10414 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk991(t5, t10296, t10298, t10301, t10309, t10310, t10313, t10410, t2242, t2247, t2248, t2315, t603, t644, t91);
        let (t10415, t10416, t10426, t10428) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk992(t10414, t117, t116, t2319, t10194, t10259, t1312, t2322, t2371, t5523, t670, t2389, t705);
    (t10381, t10389, t10392, t10398, t10401, t10407, t10410, t10414, t10415, t10416, t10426, t10428)
}
