//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1196;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1197;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1198;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1199;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1200;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1201;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1202;
use chunk7::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1203;
use chunk8::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1204;
use chunk9::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta369<F: Float>(t45: F, t57: F, t5819: F, t633: F, t5825: F, t80: F, t18281: F, t4186: F, t4328: F, t606: F, t766: F, t637: F, t83: F, t4335: F, t770: F, zeta_threshold: F, t124: F, t800: F, t828: F, t855: F, t221: F, t2675: F, t5962: F, t2674: F, t10756: F, t10758: F, t10762: F, t14836: F, t14837: F, t14839: F, t14846: F, t14850: F, t14859: F, t14864: F, t799: F, t851: F, t243: F, t6016: F, t231: F, t2662: F, t2661: F, t5977: F, t2723: F, t10726: F, t10703: F, t5966: F, t125: F, t10786: F, t2747: F, t2485: F, t6022: F, t10850: F, t775: F, t2477: F, t14718: F, t6035: F, t2749: F, t14866: F, t14871: F, t2745: F, t4362: F, t4364: F, t4366: F, t2741: F, t5980: F, t4365: F, t4424: F, t837: F, t10770: F, t2652: F, t5993: F, t14586: F, t14786: F, t14791: F, t1559: F, t4433: F, t14785: F, t6030: F, t10858: F, t6024: F, t10816: F, t10824: F, t10826: F, t6019: F, t10698: F, t1544: F, t4343: F, t5984: F, t5988: F, t1548: F, t10811: F, t6037: F, t10846: F, t10885: F, t10888: F, t10891: F, t10900: F, t2730: F, t10871: F, t836: F, t5978: F, t2484: F, t10552: F, t10554: F, t14317: F, t18261: F, t18262: F, t18265: F, t18267: F, t18300: F, t18301: F, t18308: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18378, t18390) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1196::<F>(t45, t57, t5819, t633, t5825, t80, t18281, t4186, t4328, t606, t766, t637, t83, t4335, t770, zeta_threshold);
        let (t18392, t18405) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1197::<F>(t18378, t18390, t124, t800, t828, t855, t221, t2675, t5962, t2674, t10756, t10758, t10762, t14836, t14837, t14839, t14846, t14850, t14859, t14864, t799, t851);
        let (t18411, t18416, t18420, t18424) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1198::<F>(t243, t6016, t231, t2662, t2661, t5977, t2723, t10726, t10703, t221, t5966, t2674);
        let (t18426, t18428, t18433, t18437) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1199::<F>(t125, t5977, t10786, t2747, t221, t2485, t6022, t10850, t5962, t775, t2477, t828);
        let (t18444, t18454) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1200::<F>(t14718, t6035, t2662, t2661, t125, t6016, t2747, t2749, t18426, t14866, t14871, t18411, t18416, t18420, t18424, t18428, t18433, t18437, t2745, t4362, t851);
        let (t18456, t18459, t18462, t18466, t18471, t18475) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1201::<F>(t18426, t4364, t4366, t2741, t5980, t4365, t4424, t837, t125, t5966, t10770, t2652, t5993);
        let t18489 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1202::<F>(t14586, t14786, t14791, t1559, t4433, t14785, t2652, t6030, t10858, t6024, t10816, t10824, t10826, t18456, t18459, t18462, t18466, t18471, t18475, t2745, t4362);
        let (t18491, t18495, t18500, t18507, t18511) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1203::<F>(t2741, t6019, t5966, t775, t10698, t828, t1544, t4343, t2477, t5984, t800, t5988);
        let t18524 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1204::<F>(t1548, t4343, t800, t10811, t6037, t18444, t4364, t4366, t10846, t10885, t10888, t10891, t10900, t18491, t18495, t18500, t18507, t18511, t2730, t4362, t851);
        let (t18525, t18527, t18532, t18534) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1205::<F>(t10871, t836, t18426, t4364, t221, t2485, t5978, t2484, t10552, t10554, t14317, t18261, t18262, t18265, t18267, t18300, t18301, t18308, t9278, t9308, t9316, t9329, t9333);
    (t18392, t18405, t18444, t18454, t18489, t18524, t18525, t18527, t18532, t18534)
}
