//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1156;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1157;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1158;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1159;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1160;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta331(t231: f64, t23167: f64, t10552: f64, t10554: f64, t23096: f64, t23097: f64, t23102: f64, t23103: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64, t18556: f64, t10566: f64, t23104: f64, t23106: f64, t23110: f64, t23123: f64, t23127: f64, t23128: f64, t23129: f64, t23130: f64, t9394: f64, t18563: f64, t4311: f64, t5999: f64, t10568: f64, t10577: f64, t10582: f64, t10584: f64, t10586: f64, t9514: f64, t9517: f64, t9521: f64, t9524: f64, t45: f64, t57: f64, t14441: f64, t10446: f64, t22671: f64, t22688: f64, t4377: f64, t5825: f64, t78: f64, t10457: f64, t4384: f64, t81: f64, t162: f64, t187: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t23177 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1156(t231, t23167);
        let (t23185, t23186) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1157(t10552, t10554, t23096, t23097, t23102, t23103, t9278, t9308, t9316, t9329, t9333, t18556);
        let (t23187, t23189) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1158(t10566, t23104, t23106, t23110, t23123, t23127, t23128, t23129, t23130, t23186, t9394, t18563);
        let (t23191, t23192) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1159(t4311, t5999, t10568, t10577, t10582, t10584, t10586, t23189, t9514, t9517, t9521, t9524);
        let (t23193, t23210, t23211, t23213) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1160(t45, t57, t14441, t10446, t22671, t22688, t4377, t5825, t78, t10457, t4384, t81, t162, t187, zeta_threshold);
    (t23177, t23185, t23186, t23187, t23189, t23191, t23192, t23193, t23210, t23211, t23213)
}
