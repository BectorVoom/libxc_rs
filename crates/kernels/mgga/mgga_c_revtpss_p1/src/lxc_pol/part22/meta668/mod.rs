//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta668 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2631;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2632;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2633;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta668<F: Float>(t1250: F, t21164: F, t3720: F, t140: F, t6652: F, t1222: F, t20795: F, t3629: F, t3626: F, t1261: F, t17412: F, t17444: F, t17447: F, t17453: F, t17474: F, t1808: F, t21153: F, t21157: F, t21161: F, t3625: F, t3647: F, t3718: F, t5331: F, t6673: F, t1234: F, t6594: F, t1214: F, t5825: F, t5296: F, t1042: F, t3172: F, t6630: F, t3600: F, t247: F, t3634: F, t6425: F, t1238: F, t12882: F, t12893: F, t12900: F, t12905: F, t12985: F, t17509: F, t17546: F, t17556: F, t3711: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t21165, t21166, t21172, t21173, t21176) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2631::<F>(t1250, t21164, t3720, t140, t6652, t1222, t20795, t3629, t3626, t1261, t17412, t17444, t17447, t17453, t17474, t1808, t21153, t21157, t21161, t3625, t3647, t3718, t5331, t6673);
        let (t21177, t21182, t21183, t21184, t21188, t21189, t21192) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2632::<F>(t1234, t6594, t1214, t5825, t5296, t1042, t3172, t6630, t3600, t247, t3634, t6425);
        let t21196 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2633::<F>(t1261, t21192, t1238, t12882, t12893, t12900, t12905, t12985, t17509, t17546, t17556, t21177, t21184, t21189, t3711);
    (t21165, t21166, t21172, t21173, t21176, t21177, t21182, t21183, t21184, t21188, t21192, t21196)
}
