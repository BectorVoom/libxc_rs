//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta780 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2586;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta780(t1261: f64, t12879: f64, t247: f64, t5056: f64, t225: f64, t56587: f64, t480: f64, t1214: f64, t3604: f64, t29048: f64, t3362: f64, t3655: f64, t5258: f64, t5262: f64, t12966: f64, t1803: f64, t17235: f64, t372: f64, t1284: f64, t17306: f64, t3624: f64, t12898: f64, t1804: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59233, t59241, t59242, t59279, t59330, t59336) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2586(t1261, t12879, t247, t5056, t225, t56587, t480, t1214, t3604, t29048, t3362, t3655, t5258);
        let (t59337, t59339, t59355, t59362, t59411, t59419) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2587(t59336, t3655, t5262, t12966, t1803, t17235, t372, t1284, t17306, t3624, t12898, t1804);
    (t59233, t59241, t59242, t59279, t59330, t59337, t59339, t59355, t59362, t59411, t59419)
}
