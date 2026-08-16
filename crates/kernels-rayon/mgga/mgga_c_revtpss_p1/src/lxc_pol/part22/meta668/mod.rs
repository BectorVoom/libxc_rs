//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta668 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2631;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2632;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2633;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta668(t1250: f64, t21164: f64, t3720: f64, t140: f64, t6652: f64, t1222: f64, t20795: f64, t3629: f64, t3626: f64, t1261: f64, t17412: f64, t17444: f64, t17447: f64, t17453: f64, t17474: f64, t1808: f64, t21153: f64, t21157: f64, t21161: f64, t3625: f64, t3647: f64, t3718: f64, t5331: f64, t6673: f64, t1234: f64, t6594: f64, t1214: f64, t5825: f64, t5296: f64, t1042: f64, t3172: f64, t6630: f64, t3600: f64, t247: f64, t3634: f64, t6425: f64, t1238: f64, t12882: f64, t12893: f64, t12900: f64, t12905: f64, t12985: f64, t17509: f64, t17546: f64, t17556: f64, t3711: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21165, t21166, t21172, t21173, t21176) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2631(t1250, t21164, t3720, t140, t6652, t1222, t20795, t3629, t3626, t1261, t17412, t17444, t17447, t17453, t17474, t1808, t21153, t21157, t21161, t3625, t3647, t3718, t5331, t6673);
        let (t21177, t21182, t21183, t21184, t21188, t21189, t21192) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2632(t1234, t6594, t1214, t5825, t5296, t1042, t3172, t6630, t3600, t247, t3634, t6425);
        let t21196 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2633(t1261, t21192, t1238, t12882, t12893, t12900, t12905, t12985, t17509, t17546, t17556, t21177, t21184, t21189, t3711);
    (t21165, t21166, t21172, t21173, t21176, t21177, t21182, t21183, t21184, t21188, t21192, t21196)
}
