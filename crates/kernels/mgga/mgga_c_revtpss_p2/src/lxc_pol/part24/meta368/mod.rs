//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1250;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1251;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1252;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1253;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1254;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta368<F: Float>(t12459: F, t12460: F, t24238: F, t24242: F, t24246: F, t24250: F, t24289: F, t24292: F, t24295: F, t24298: F, t24313: F, t24315: F, t24318: F, t24320: F, t24348: F, t1169: F, t12472: F, t24330: F, t1756: F, t6518: F, t3523: F, t16706: F, t16876: F, t20276: F, t20278: F, t20280: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24265: F, t24267: F, t24272: F, t24275: F, t12542: F, t12543: F, t1188: F, t12555: F, t20671: F, t1745: F, t6502: F, t1744: F, t20618: F, t1757: F, t6534: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t24361 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1250::<F>(t12459, t12460, t24238, t24242, t24246, t24250, t24289, t24292, t24295, t24298, t24313, t24315, t24318, t24320);
        let (t24362, t24363, t24366, t24375) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1251::<F>(t24348, t24361, t1169, t12472, t24330, t1756, t6518);
        let (t24376, t24393) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1252::<F>(t24375, t3523, t16706, t16876, t20276, t20278, t20280, t20283, t20285, t20287, t24230, t24234, t24265, t24267, t24272, t24275);
        let t24406 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1253::<F>(t12542, t12543, t24238, t24242, t24246, t24250, t24289, t24292, t24295, t24298, t24313, t24315, t24318, t24320);
        let (t24407, t24408, t24411, t24414, t24417, t24420, t24423) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1254::<F>(t24393, t24406, t1188, t12555, t24375, t1756, t20671, t1745, t6502, t1744, t20618, t1757, t6534);
    (t24362, t24363, t24366, t24375, t24376, t24407, t24408, t24411, t24414, t24417, t24420, t24423)
}
