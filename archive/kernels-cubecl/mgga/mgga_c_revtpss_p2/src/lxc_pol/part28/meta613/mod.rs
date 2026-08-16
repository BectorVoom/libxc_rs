//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2141;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2142;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2143;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2144;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta613<F: Float>(t13648: F, t2014: F, t7312: F, t25861: F, t7732: F, t2322: F, t28056: F, t25194: F, t7898: F, t25851: F, t10416: F, t7735: F, t13435: F, t27137: F, t1453: F, t1518: F, t25800: F, t28230: F, t651: F, t98567: F, t98569: F, t98571: F, t98574: F, t98578: F, t98581: F, t98584: F, t98590: F, t98594: F, t25856: F, t4248: F, t2034: F, t49564: F, t2033: F, t3829: F, t7900: F, t28067: F, t95088: F, t14468: F, t30: F, t2: F, t2411: F, t580: F, t890: F, t27382: F, t198: F, t206: F, t7782: F, t892: F, t775: F, t25206: F, t1583: F, t2430: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98597, t98599, t98601, t98603, t98605, t98607) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2141::<F>(t13648, t2014, t7312, t25861, t7732, t2322, t28056, t25194, t7898, t25851, t10416, t7735);
        let t98612 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2142::<F>(t13435, t7735, t2322, t27137, t1453, t1518, t25800, t28230, t651, t98567, t98569, t98571, t98574, t98578, t98581, t98584, t98590, t98594, t98597, t98599, t98601, t98603, t98605, t98607);
        let (t98615, t98617, t98621, t98623, t98627) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2143::<F>(t25856, t4248, t2014, t2034, t49564, t2033, t3829, t7900, t28067, t95088, t14468, t30);
        let (t98635, t98637, t98650, t98651) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2144::<F>(t2, t2411, t580, t890, t27382, t198, t206, t7782, t892, t775, t25206, t1583, t2430);
    (t98612, t98615, t98617, t98621, t98623, t98627, t98635, t98637, t98650, t98651)
}
