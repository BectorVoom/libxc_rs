//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta891 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2841;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2842;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2843;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2844;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2845;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta891(t23342: f64, t2652: f64, t221: f64, t23114: f64, t2674: f64, t40683: f64, t14648: f64, t14832: f64, t2661: f64, t5962: f64, t23346: f64, t231: f64, t2662: f64, t76569: f64, t23244: f64, t243: f64, t10871: f64, t40693: f64, t23263: f64, t40864: f64, t40462: f64, t40810: f64, t51042: f64, t51055: f64, t62108: f64, t62111: f64, t62114: f64, t62129: f64, t62135: f64, t62148: f64, t775: f64, t828: f64, t851: f64, t10697: f64, t236: f64, t807: f64, t23267: f64, t2703: f64, t40850: f64, t51059: f64, t51061: f64, t51074: f64, t51079: f64, t51081: f64, t51083: f64, t51086: f64, t51089: f64, t51093: f64, t51096: f64, t62162: f64, t62168: f64, t62176: f64, t62178: f64, t62188: f64, t23148: f64, t854: f64, t1559: f64, t18599: f64, t40862: f64, t51099: f64, t51100: f64, t51102: f64, t51104: f64, t51122: f64, t51170: f64, t62216: f64, t62236: f64, t62241: f64, t62246: f64, t62251: f64, t62392: f64, t62399: f64, t62401: f64, t62405: f64, t45: f64, t23177: f64, t2484: f64, t2485: f64, t14325: f64, t23216: f64, t1469: f64, t4401: f64, t61303: f64, t14401: f64, t14404: f64, t18272: f64, t18281: f64, t19680: f64, t22671: f64, t22688: f64, t2375: f64, t39825: f64, t4186: f64, t4377: f64, t5825: f64, t606: f64, t76397: f64, t78: f64, zeta_threshold: f64, t57: f64, t14413: f64, t14416: f64, t18286: f64, t2382: f64, t39840: f64, t4384: f64, t81: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76804, t76808, t76812, t76814, t76818) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2841(t23342, t2652, t221, t23114, t2674, t40683, t14648, t14832, t2661, t5962, t23346, t231, t2662, t76569);
        let t76843 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2842(t231, t23244, t243, t2661, t2662, t10871, t40693, t76569, t23263, t40864, t23114, t40462, t40810, t51042, t51055, t62108, t62111, t62114, t62129, t62135, t62148, t76804, t76808, t76812, t76814, t76818, t775, t828, t851);
        let t76860 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2843(t10697, t23114, t236, t807, t23267, t2703, t40850, t51059, t51061, t51074, t51079, t51081, t51083, t51086, t51089, t51093, t51096, t62162, t62168, t62176, t62178, t62188);
        let t76884 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2844(t23148, t236, t807, t854, t1559, t18599, t2661, t2662, t40862, t51099, t51100, t51102, t51104, t51122, t51170, t62216, t62236, t62241, t62246, t62251, t62392, t62399, t62401, t62405);
        let (t76887, t76890, t76893, t76911) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2845(t45, t221, t23177, t2484, t2485, t14325, t23216, t1469, t4401, t61303, t14401, t14404, t18272, t18281, t19680, t22671, t22688, t2375, t39825, t4186, t4377, t5825, t606, t76397, t78, zeta_threshold);
        let t76929 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2846(t57, t14413, t14416, t18281, t18286, t19680, t22671, t22688, t2382, t39840, t4186, t4384, t5825, t606, t76397, t81, zeta_threshold);
    (t76843, t76860, t76884, t76887, t76890, t76893, t76911, t76929)
}
