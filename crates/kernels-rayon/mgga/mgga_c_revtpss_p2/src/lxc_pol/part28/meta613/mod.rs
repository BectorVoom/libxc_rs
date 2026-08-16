//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2141;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2142;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2143;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2144;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta613(t13648: f64, t2014: f64, t7312: f64, t25861: f64, t7732: f64, t2322: f64, t28056: f64, t25194: f64, t7898: f64, t25851: f64, t10416: f64, t7735: f64, t13435: f64, t27137: f64, t1453: f64, t1518: f64, t25800: f64, t28230: f64, t651: f64, t98567: f64, t98569: f64, t98571: f64, t98574: f64, t98578: f64, t98581: f64, t98584: f64, t98590: f64, t98594: f64, t25856: f64, t4248: f64, t2034: f64, t49564: f64, t2033: f64, t3829: f64, t7900: f64, t28067: f64, t95088: f64, t14468: f64, t30: f64, t2: f64, t2411: f64, t580: f64, t890: f64, t27382: f64, t198: f64, t206: f64, t7782: f64, t892: f64, t775: f64, t25206: f64, t1583: f64, t2430: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98597, t98599, t98601, t98603, t98605, t98607) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2141(t13648, t2014, t7312, t25861, t7732, t2322, t28056, t25194, t7898, t25851, t10416, t7735);
        let t98612 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2142(t13435, t7735, t2322, t27137, t1453, t1518, t25800, t28230, t651, t98567, t98569, t98571, t98574, t98578, t98581, t98584, t98590, t98594, t98597, t98599, t98601, t98603, t98605, t98607);
        let (t98615, t98617, t98621, t98623, t98627) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2143(t25856, t4248, t2014, t2034, t49564, t2033, t3829, t7900, t28067, t95088, t14468, t30);
        let (t98635, t98637, t98650, t98651) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2144(t2, t2411, t580, t890, t27382, t198, t206, t7782, t892, t775, t25206, t1583, t2430);
    (t98612, t98615, t98617, t98621, t98623, t98627, t98635, t98637, t98650, t98651)
}
