//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1660;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1661;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1662;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1663;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta457<F: Float>(t1234: F, t6594: F, t1214: F, t5825: F, t5296: F, t1042: F, t3172: F, t6630: F, t3600: F, t247: F, t3634: F, t6425: F, t1261: F, t1238: F, t12882: F, t12893: F, t12900: F, t12905: F, t12985: F, t17509: F, t17546: F, t17556: F, t3711: F, t20721: F, t3719: F, t3670: F, t5390: F, t1225: F, t18281: F, t1012: F, t1010: F, t5843: F, t5378: F, t5381: F, t21040: F, t3629: F, t3626: F, t12840: F, t20795: F, t1222: F, t1227: F, t13012: F, t17593: F, t17619: F, t17622: F, t3625: F, t5340: F, t5369: F, t5373: F, t5384: F, t5386: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t21177, t21184, t21188, t21189, t21192) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1660::<F>(t1234, t6594, t1214, t5825, t5296, t1042, t3172, t6630, t3600, t247, t3634, t6425);
        let t21196 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1661::<F>(t1261, t21192, t1238, t12882, t12893, t12900, t12905, t12985, t17509, t17546, t17556, t21177, t21184, t21189, t3711);
        let (t21200, t21203, t21210, t21213, t21216) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1662::<F>(t20721, t247, t3719, t3670, t5390, t1225, t18281, t1012, t1010, t5843, t5378, t5381);
        let (t21219, t21223, t21226) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1663::<F>(t21040, t3629, t3626, t12840, t20795, t1222, t1227, t13012, t17593, t17619, t17622, t21200, t21203, t21210, t21213, t21216, t3625, t5340, t5369, t5373, t5384, t5386);
    (t21184, t21188, t21192, t21196, t21200, t21210, t21219, t21223, t21226)
}
