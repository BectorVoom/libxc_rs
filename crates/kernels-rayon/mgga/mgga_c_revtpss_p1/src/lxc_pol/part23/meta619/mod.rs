//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2296;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2297;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2298;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta619(t24236: f64, t5312: f64, t13046: f64, t24544: f64, t1042: f64, t13053: f64, t1803: f64, t6601: f64, t1222: f64, t1235: f64, t1261: f64, t12853: f64, t13042: f64, t13052: f64, t1797: f64, t21053: f64, t21088: f64, t21091: f64, t21102: f64, t24636: f64, t24640: f64, t24644: f64, t24649: f64, t24652: f64, t3711: f64, t484: f64, t476: f64, t52: f64, t475: f64, t467: f64, t1785: f64, t6594: f64, t12678: f64, t16706: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64, t459: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24663, t24664, t24667, t24668, t24671, t24674) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2296(t24236, t5312, t13046, t24544, t1042, t13053, t1803, t6601, t1222, t1235, t1261, t12853, t13042, t13052, t1797, t21053, t21088, t21091, t21102, t24636, t24640, t24644, t24649, t24652, t3711, t484);
        let (t24679, t24680, t24681, t24684, t24697) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2297(t476, t52, t475, t467, t1785, t6594, t12678, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250);
        let t24698 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2298(t24697, t459);
    (t24663, t24664, t24667, t24668, t24671, t24674, t24679, t24680, t24681, t24684, t24697, t24698)
}
