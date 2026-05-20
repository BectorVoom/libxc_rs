//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta780 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2586;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta780<F: Float>(t1261: F, t12879: F, t247: F, t5056: F, t225: F, t56587: F, t480: F, t1214: F, t3604: F, t29048: F, t3362: F, t3655: F, t5258: F, t5262: F, t12966: F, t1803: F, t17235: F, t372: F, t1284: F, t17306: F, t3624: F, t12898: F, t1804: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t59233, t59241, t59242, t59279, t59330, t59336) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2586::<F>(t1261, t12879, t247, t5056, t225, t56587, t480, t1214, t3604, t29048, t3362, t3655, t5258);
        let (t59337, t59339, t59355, t59362, t59411, t59419) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2587::<F>(t59336, t3655, t5262, t12966, t1803, t17235, t372, t1284, t17306, t3624, t12898, t1804);
    (t59233, t59241, t59242, t59279, t59330, t59337, t59339, t59355, t59362, t59411, t59419)
}
