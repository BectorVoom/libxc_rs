//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2036;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2037;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2038;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2039;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta522(t17737: f64, t5297: f64, t3626: f64, t1230: f64, t6594: f64, t1803: f64, t5261: f64, t12297: f64, t12678: f64, t16706: f64, t17319: f64, t17320: f64, t17321: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64, t459: f64, t225: f64, t480: f64, t12832: f64, t17401: f64, t17736: f64, t17767: f64, t17771: f64, t17791: f64, t17792: f64, t21300: f64, t21306: f64, t3718: f64, t484: f64, t5335: f64, t5348: f64, t6690: f64, t20782: f64, t20828: f64, t20855: f64, t20910: f64, t20955: f64, t20993: f64, t21027: f64, t21057: f64, t21114: f64, t21146: f64, t21176: f64, t21196: f64, t21226: f64, t21264: f64, t21295: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21309, t21310, t21313, t21316, t21332) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2036(t17737, t5297, t3626, t1230, t6594, t1803, t5261, t12297, t12678, t16706, t17319, t17320, t17321, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let t21333 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2037(t21332, t459);
        let (t21334, t21335, t21338) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2038(t21333, t225, t480, t12832, t17401, t17736, t17767, t17771, t17791, t17792, t21300, t21306, t21310, t21313, t21316, t3718, t484, t5335, t5348, t6690);
        let t21342 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2039(t20782, t20828, t20855, t20910, t20955, t20993, t21027, t21057, t21114, t21146, t21176, t21196, t21226, t21264, t21295, t21338);
    (t21309, t21310, t21313, t21316, t21332, t21333, t21334, t21335, t21342)
}
