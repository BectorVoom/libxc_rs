//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2024;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2025;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta517<F: Float>(t1214: F, t5825: F, t5296: F, t1042: F, t3172: F, t6630: F, t3600: F, t247: F, t3634: F, t6425: F, t1261: F, t1238: F, t12882: F, t12893: F, t12900: F, t12905: F, t12985: F, t17509: F, t17546: F, t17556: F, t21177: F, t3711: F, t20721: F, t3719: F, t3670: F, t5390: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21182, t21183, t21184, t21188, t21189, t21192, t21193, t21196) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2024::<F>(t1214, t5825, t5296, t1042, t3172, t6630, t3600, t247, t3634, t6425, t1261, t1238, t12882, t12893, t12900, t12905, t12985, t17509, t17546, t17556, t21177, t3711);
        let (t21200, t21203) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2025::<F>(t20721, t247, t3719, t3670, t5390);
    (t21182, t21183, t21184, t21188, t21189, t21192, t21193, t21196, t21200, t21203)
}
