//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2036;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2037;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2038;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2039;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta522<F: Float>(t17737: F, t5297: F, t3626: F, t1230: F, t6594: F, t1803: F, t5261: F, t12297: F, t12678: F, t16706: F, t17319: F, t17320: F, t17321: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F, t459: F, t225: F, t480: F, t12832: F, t17401: F, t17736: F, t17767: F, t17771: F, t17791: F, t17792: F, t21300: F, t21306: F, t3718: F, t484: F, t5335: F, t5348: F, t6690: F, t20782: F, t20828: F, t20855: F, t20910: F, t20955: F, t20993: F, t21027: F, t21057: F, t21114: F, t21146: F, t21176: F, t21196: F, t21226: F, t21264: F, t21295: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t21309, t21310, t21313, t21316, t21332) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2036::<F>(t17737, t5297, t3626, t1230, t6594, t1803, t5261, t12297, t12678, t16706, t17319, t17320, t17321, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let t21333 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2037::<F>(t21332, t459);
        let (t21334, t21335, t21338) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2038::<F>(t21333, t225, t480, t12832, t17401, t17736, t17767, t17771, t17791, t17792, t21300, t21306, t21310, t21313, t21316, t3718, t484, t5335, t5348, t6690);
        let t21342 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2039::<F>(t20782, t20828, t20855, t20910, t20955, t20993, t21027, t21057, t21114, t21146, t21176, t21196, t21226, t21264, t21295, t21338);
    (t21309, t21310, t21313, t21316, t21332, t21333, t21334, t21335, t21342)
}
