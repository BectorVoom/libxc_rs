//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1505;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1506;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta501<F: Float>(t1558: F, t5962: F, t10777: F, t14671: F, t14686: F, t6017: F, t10811: F, t23293: F, t1544: F, t23327: F, t23323: F, t14586: F, t14931: F, t61715: F, t221: F, t23148: F, t2674: F, t2675: F, t23297: F, t14923: F, t23336: F, t23167: F, t243: F, t10726: F, t2661: F, t2723: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t76302, t76313, t76315, t76321, t76330, t76337, t76362) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1505::<F>(t1558, t5962, t10777, t14671, t14686, t6017, t10811, t23293, t1544, t23327, t23323, t14586, t14931, t61715);
        let (t76428, t76500, t76502, t76569, t76572) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1506::<F>(t221, t23148, t2674, t2675, t10811, t23297, t14923, t23336, t23167, t243, t10726, t2661, t2723);
    (t76302, t76313, t76315, t76321, t76330, t76337, t76362, t76428, t76500, t76502, t76569, t76572)
}
