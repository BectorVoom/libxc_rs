//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1109;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1110;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1111;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1112;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1113;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1114;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1115;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta263(t7427: f64, t892: f64, t2070: f64, t2411: f64, t30: f64, t265: f64, t393: f64, t207: f64, t1940: f64, t198: f64, t2071: f64, t2403: f64, t775: f64, t890: f64, t2078: f64, t45: f64, t605: f64, t606: f64, t7010: f64, t7092: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t33: f64, t502: f64, t1113: f64, t2085: f64, t57: f64, t7200: f64, t7207: f64, rho1: f64, t1312: f64, t2055: f64, t2322: f64, t5523: f64, t670: f64, t7357: f64, t7359: f64, t7373: f64, t2106: f64, t531: f64, t7238: f64, t2097: f64, t212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7428 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1109(t7427, t892);
        let t7432 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1110(t2070, t2411);
        let (t7448, t7449, t7454) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1111(t30, t265, t393, t207, t7427, t1940, t198, t2071, t2403, t7432, t775, t890, t892, t2078, t45, t605, t606, t7010, t7092, t7428, dens_threshold, rho0, zeta_threshold);
        let (t7468, t7473) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1112(t33, t265, t502, t7448, t1113, t1940, t2071, t2085, t2403, t57, t606, t7200, t7207, t7428, t7432, dens_threshold, rho1, zeta_threshold);
        let t7474 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1113(t7454, t7473);
        let (t7484, t7488) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1114(t1312, t2055, t2322, t5523, t670, t7357, t7359, t7373, t2106, t531);
        let (t7489, t7492) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1115(t7238, t7488, t2097, t212);
    (t7428, t7432, t7449, t7468, t7474, t7484, t7488, t7489, t7492)
}
