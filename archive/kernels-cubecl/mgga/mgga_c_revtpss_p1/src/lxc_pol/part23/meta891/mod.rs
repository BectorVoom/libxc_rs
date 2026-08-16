//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta891 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2841;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2842;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2843;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2844;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2845;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta891<F: Float>(t23342: F, t2652: F, t221: F, t23114: F, t2674: F, t40683: F, t14648: F, t14832: F, t2661: F, t5962: F, t23346: F, t231: F, t2662: F, t76569: F, t23244: F, t243: F, t10871: F, t40693: F, t23263: F, t40864: F, t40462: F, t40810: F, t51042: F, t51055: F, t62108: F, t62111: F, t62114: F, t62129: F, t62135: F, t62148: F, t775: F, t828: F, t851: F, t10697: F, t236: F, t807: F, t23267: F, t2703: F, t40850: F, t51059: F, t51061: F, t51074: F, t51079: F, t51081: F, t51083: F, t51086: F, t51089: F, t51093: F, t51096: F, t62162: F, t62168: F, t62176: F, t62178: F, t62188: F, t23148: F, t854: F, t1559: F, t18599: F, t40862: F, t51099: F, t51100: F, t51102: F, t51104: F, t51122: F, t51170: F, t62216: F, t62236: F, t62241: F, t62246: F, t62251: F, t62392: F, t62399: F, t62401: F, t62405: F, t45: F, t23177: F, t2484: F, t2485: F, t14325: F, t23216: F, t1469: F, t4401: F, t61303: F, t14401: F, t14404: F, t18272: F, t18281: F, t19680: F, t22671: F, t22688: F, t2375: F, t39825: F, t4186: F, t4377: F, t5825: F, t606: F, t76397: F, t78: F, zeta_threshold: F, t57: F, t14413: F, t14416: F, t18286: F, t2382: F, t39840: F, t4384: F, t81: F) -> (F, F, F, F, F, F, F, F) {
        let (t76804, t76808, t76812, t76814, t76818) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2841::<F>(t23342, t2652, t221, t23114, t2674, t40683, t14648, t14832, t2661, t5962, t23346, t231, t2662, t76569);
        let t76843 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2842::<F>(t231, t23244, t243, t2661, t2662, t10871, t40693, t76569, t23263, t40864, t23114, t40462, t40810, t51042, t51055, t62108, t62111, t62114, t62129, t62135, t62148, t76804, t76808, t76812, t76814, t76818, t775, t828, t851);
        let t76860 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2843::<F>(t10697, t23114, t236, t807, t23267, t2703, t40850, t51059, t51061, t51074, t51079, t51081, t51083, t51086, t51089, t51093, t51096, t62162, t62168, t62176, t62178, t62188);
        let t76884 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2844::<F>(t23148, t236, t807, t854, t1559, t18599, t2661, t2662, t40862, t51099, t51100, t51102, t51104, t51122, t51170, t62216, t62236, t62241, t62246, t62251, t62392, t62399, t62401, t62405);
        let (t76887, t76890, t76893, t76911) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2845::<F>(t45, t221, t23177, t2484, t2485, t14325, t23216, t1469, t4401, t61303, t14401, t14404, t18272, t18281, t19680, t22671, t22688, t2375, t39825, t4186, t4377, t5825, t606, t76397, t78, zeta_threshold);
        let t76929 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2846::<F>(t57, t14413, t14416, t18281, t18286, t19680, t22671, t22688, t2382, t39840, t4186, t4384, t5825, t606, t76397, t81, zeta_threshold);
    (t76843, t76860, t76884, t76887, t76890, t76893, t76911, t76929)
}
