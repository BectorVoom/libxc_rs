//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta949 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3189;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta949<F: Float>(t29048: F, t3362: F, t3655: F, t5258: F, t5262: F, t12976: F, t5362: F, t12963: F, t5327: F, t12995: F, t17308: F, t17283: F, t3678: F, t12901: F, t17572: F, t17235: F, t372: F, t13068: F, t5292: F, t1032: F, t1246: F, t17331: F, t1247: F, t17221: F, t3172: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t59330, t59336, t59338, t59349, t59351, t59353, t59358) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3189::<F>(t29048, t3362, t3655, t5258, t5262, t12976, t5362, t12963, t5327, t12995, t17308, t17283, t3678);
        let (t59360, t59362, t59371, t59375, t59379) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3190::<F>(t12901, t17572, t17235, t372, t13068, t5292, t1032, t1246, t17331, t1247, t17221, t3172);
    (t59330, t59336, t59338, t59349, t59351, t59353, t59358, t59360, t59362, t59371, t59375, t59379)
}
