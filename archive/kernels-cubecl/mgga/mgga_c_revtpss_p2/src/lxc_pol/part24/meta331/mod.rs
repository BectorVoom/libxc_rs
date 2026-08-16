//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1156;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1157;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1158;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1159;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1160;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta331<F: Float>(t231: F, t23167: F, t10552: F, t10554: F, t23096: F, t23097: F, t23102: F, t23103: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F, t18556: F, t10566: F, t23104: F, t23106: F, t23110: F, t23123: F, t23127: F, t23128: F, t23129: F, t23130: F, t9394: F, t18563: F, t4311: F, t5999: F, t10568: F, t10577: F, t10582: F, t10584: F, t10586: F, t9514: F, t9517: F, t9521: F, t9524: F, t45: F, t57: F, t14441: F, t10446: F, t22671: F, t22688: F, t4377: F, t5825: F, t78: F, t10457: F, t4384: F, t81: F, t162: F, t187: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t23177 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1156::<F>(t231, t23167);
        let (t23185, t23186) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1157::<F>(t10552, t10554, t23096, t23097, t23102, t23103, t9278, t9308, t9316, t9329, t9333, t18556);
        let (t23187, t23189) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1158::<F>(t10566, t23104, t23106, t23110, t23123, t23127, t23128, t23129, t23130, t23186, t9394, t18563);
        let (t23191, t23192) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1159::<F>(t4311, t5999, t10568, t10577, t10582, t10584, t10586, t23189, t9514, t9517, t9521, t9524);
        let (t23193, t23210, t23211, t23213) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1160::<F>(t45, t57, t14441, t10446, t22671, t22688, t4377, t5825, t78, t10457, t4384, t81, t162, t187, zeta_threshold);
    (t23177, t23185, t23186, t23187, t23189, t23191, t23192, t23193, t23210, t23211, t23213)
}
