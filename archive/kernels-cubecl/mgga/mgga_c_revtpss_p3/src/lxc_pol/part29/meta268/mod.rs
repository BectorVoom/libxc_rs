//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta268 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1108;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1109;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1110;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1111;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1112;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1113;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta268<F: Float>(t7427: F, t892: F, t2070: F, t2411: F, t30: F, t265: F, t393: F, t207: F, t1940: F, t198: F, t2071: F, t2403: F, t775: F, t890: F, t2078: F, t45: F, t605: F, t606: F, t7010: F, t7092: F, dens_threshold: F, rho0: F, zeta_threshold: F, t33: F, t502: F, t1113: F, t2085: F, t57: F, t7200: F, t7207: F, rho1: F, t1312: F, t2055: F, t2322: F, t5523: F, t670: F, t7357: F, t7359: F, t7373: F, t2106: F, t531: F, t7238: F, t2097: F, t212: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t7428 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1108::<F>(t7427, t892);
        let t7432 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1109::<F>(t2070, t2411);
        let (t7443, t7448, t7449, t7454) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1110::<F>(t30, t265, t393, t207, t7427, t1940, t198, t2071, t2403, t7432, t775, t890, t892, t2078, t45, t605, t606, t7010, t7092, t7428, dens_threshold, rho0, zeta_threshold);
        let (t7468, t7473) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1111::<F>(t33, t265, t502, t7448, t1113, t1940, t2071, t2085, t2403, t57, t606, t7200, t7207, t7428, t7432, dens_threshold, rho1, zeta_threshold);
        let t7474 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1112::<F>(t7454, t7473);
        let (t7484, t7488) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1113::<F>(t1312, t2055, t2322, t5523, t670, t7357, t7359, t7373, t2106, t531);
        let (t7489, t7492) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1114::<F>(t7238, t7488, t2097, t212);
    (t7428, t7432, t7443, t7449, t7468, t7474, t7484, t7488, t7489, t7492)
}
