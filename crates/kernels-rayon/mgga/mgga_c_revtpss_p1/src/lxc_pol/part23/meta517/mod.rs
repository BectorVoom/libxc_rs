//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2024;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2025;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta517(t1214: f64, t5825: f64, t5296: f64, t1042: f64, t3172: f64, t6630: f64, t3600: f64, t247: f64, t3634: f64, t6425: f64, t1261: f64, t1238: f64, t12882: f64, t12893: f64, t12900: f64, t12905: f64, t12985: f64, t17509: f64, t17546: f64, t17556: f64, t21177: f64, t3711: f64, t20721: f64, t3719: f64, t3670: f64, t5390: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21182, t21183, t21184, t21188, t21189, t21192, t21193, t21196) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2024(t1214, t5825, t5296, t1042, t3172, t6630, t3600, t247, t3634, t6425, t1261, t1238, t12882, t12893, t12900, t12905, t12985, t17509, t17546, t17556, t21177, t3711);
        let (t21200, t21203) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2025(t20721, t247, t3719, t3670, t5390);
    (t21182, t21183, t21184, t21188, t21189, t21192, t21193, t21196, t21200, t21203)
}
