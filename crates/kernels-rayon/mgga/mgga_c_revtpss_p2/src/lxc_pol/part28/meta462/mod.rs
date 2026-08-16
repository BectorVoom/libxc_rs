//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1761;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1762;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1763;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1764;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta462(t5: f64, t25167: f64, t117: f64, t4144: f64, t9593: f64, t2034: f64, t2014: f64, t10416: f64, t1937: f64, t13435: f64, t2322: f64, t6993: f64, t196: f64, t197: f64, t3821: f64, t2035: f64, t531: f64, t7311: f64, t7238: f64, t7312: f64, t7315: f64, t1310: f64, t1453: f64, t1932: f64, t2007: f64, t2320: f64, t2328: f64, t25078: f64, t25085: f64, t25092: f64, t25095: f64, t25096: f64, t3813: f64, t508: f64, t649: f64, t651: f64, t6983: f64, t7221: f64, t7231: f64, t2394: f64, t30: f64, t1962: f64, t198: f64, t206: f64, t2411: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25168, t25169, t25177, t25178, t25180, t25182, t25184, t25186, t25188) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1761(t5, t25167, t117, t4144, t9593, t2034, t2014, t10416, t1937, t13435, t2322, t6993, t196, t197, t3821);
        let (t25190, t25191, t25194, t25197) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1762(t2035, t25188, t531, t7311, t7238, t2014, t7312, t7315, t1310, t1453, t1932, t2007, t2320, t2328, t25078, t25085, t25092, t25095, t25096, t25169, t25180, t25182, t25184, t25186, t3813, t508, t649, t651, t6983, t7221, t7231);
        let (t25198, t25206) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1763(t2394, t30, t1962, t198, t206);
        let t25207 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1764(t2411, t30);
    (t25168, t25169, t25177, t25178, t25188, t25190, t25191, t25194, t25197, t25198, t25206, t25207)
}
