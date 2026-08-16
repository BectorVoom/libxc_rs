//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1166;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1167;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1168;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1169;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1170;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1171;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta278(t730: f64, t9446: f64, t2596: f64, t675: f64, t215: f64, t723: f64, t2553: f64, t738: f64, t2491: f64, t177: f64, t9417: f64, t2495: f64, t9368: f64, t2531: f64, t2536: f64, t2539: f64, t2549: f64, t2557: f64, t2591: f64, t2598: f64, t2601: f64, t2605: f64, t268: f64, t724: f64, t731: f64, t746: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64, t9433: f64, t9435: f64, t745: f64, t9385: f64, t2514: f64, t744: f64, t2576: f64, t2582: f64, t2584: f64, t700: f64, t2519: f64, t2577: f64, t2581: f64, t2585: f64, t2565: f64, t702: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9447, t9450, t9454, t9461, t9469, t9476, t9480, t9481) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1166(t730, t9446, t2596, t675, t215, t723, t2553, t738, t2491, t177, t9417, t2495, t9368);
        let t9484 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1167(t2531, t2536, t2539, t2549, t2557, t2591, t2598, t2601, t2605, t268, t675, t724, t731, t746, t9278, t9308, t9316, t9329, t9333, t9433, t9435, t9447, t9450, t9454, t9461, t9469, t9476, t9480, t9481);
        let (t9485, t9488, t9501, t9508, t9514) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1168(t745, t9385, t9368, t2514, t746, t2495, t744, t2576, t2582, t2584, t700);
        let t9517 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1169(t2519, t2577, t268);
        let t9521 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1170(t2581, t675, t2585, t268);
        let t9524 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1171(t2565, t2576, t702);
    (t9484, t9485, t9488, t9501, t9508, t9514, t9517, t9521, t9524)
}
