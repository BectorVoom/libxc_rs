//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta261 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1164;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1165;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1166;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1167;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1168;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1169;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta261<F: Float>(t1372: F, t7252: F, t546: F, t550: F, t7028: F, t807: F, t2018: F, t786: F, t1381: F, t1385: F, t64: F, t239: F, t820: F, t1401: F, t1405: F, t2019: F, t545: F, t1416: F, t7251: F, t225: F, t2022: F, t213: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7253, t7256, t7258, t7259, t7261, t7262) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1164::<F>(t1372, t7252, t546, t550, t7028, t807, t2018, t786, t1381, t1385, t64);
        let t7264 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1165::<F>(t239, t7262, t820);
        let (t7265, t7268, t7269) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1166::<F>(t1401, t7264, t1405, t2019, t545, t64);
        let t7271 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1167::<F>(t239, t7269, t820);
        let t7274 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1168::<F>(t1416, t7271, t7251, t7253, t7258, t7261, t7265, t7268);
        let (t7275, t7279) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1169::<F>(t225, t7274, t2022, t213);
    (t7256, t7258, t7259, t7261, t7262, t7264, t7268, t7269, t7271, t7274, t7275, t7279)
}
