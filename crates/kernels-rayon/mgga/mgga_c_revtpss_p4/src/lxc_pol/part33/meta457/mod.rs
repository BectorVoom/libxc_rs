//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1660;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1661;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1662;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1663;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta457(t1234: f64, t6594: f64, t1214: f64, t5825: f64, t5296: f64, t1042: f64, t3172: f64, t6630: f64, t3600: f64, t247: f64, t3634: f64, t6425: f64, t1261: f64, t1238: f64, t12882: f64, t12893: f64, t12900: f64, t12905: f64, t12985: f64, t17509: f64, t17546: f64, t17556: f64, t3711: f64, t20721: f64, t3719: f64, t3670: f64, t5390: f64, t1225: f64, t18281: f64, t1012: f64, t1010: f64, t5843: f64, t5378: f64, t5381: f64, t21040: f64, t3629: f64, t3626: f64, t12840: f64, t20795: f64, t1222: f64, t1227: f64, t13012: f64, t17593: f64, t17619: f64, t17622: f64, t3625: f64, t5340: f64, t5369: f64, t5373: f64, t5384: f64, t5386: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21177, t21184, t21188, t21189, t21192) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1660(t1234, t6594, t1214, t5825, t5296, t1042, t3172, t6630, t3600, t247, t3634, t6425);
        let t21196 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1661(t1261, t21192, t1238, t12882, t12893, t12900, t12905, t12985, t17509, t17546, t17556, t21177, t21184, t21189, t3711);
        let (t21200, t21203, t21210, t21213, t21216) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1662(t20721, t247, t3719, t3670, t5390, t1225, t18281, t1012, t1010, t5843, t5378, t5381);
        let (t21219, t21223, t21226) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1663(t21040, t3629, t3626, t12840, t20795, t1222, t1227, t13012, t17593, t17619, t17622, t21200, t21203, t21210, t21213, t21216, t3625, t5340, t5369, t5373, t5384, t5386);
    (t21184, t21188, t21192, t21196, t21200, t21210, t21219, t21223, t21226)
}
