//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta368 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1194;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1195;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1196;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1197;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1198;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1199;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1200;
use chunk7::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1201;
use chunk8::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1202;
use chunk9::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1203;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta368(t45: f64, t57: f64, t5819: f64, t633: f64, t5825: f64, t80: f64, t18281: f64, t4186: f64, t4328: f64, t606: f64, t766: f64, t637: f64, t83: f64, t4335: f64, t770: f64, zeta_threshold: f64, t124: f64, t800: f64, t828: f64, t855: f64, t221: f64, t2675: f64, t5962: f64, t2674: f64, t10756: f64, t10758: f64, t10762: f64, t14836: f64, t14837: f64, t14839: f64, t14846: f64, t14850: f64, t14859: f64, t14864: f64, t799: f64, t851: f64, t243: f64, t6016: f64, t231: f64, t2662: f64, t2661: f64, t5977: f64, t2723: f64, t10726: f64, t10703: f64, t5966: f64, t125: f64, t10786: f64, t2747: f64, t2485: f64, t6022: f64, t10850: f64, t775: f64, t2477: f64, t14718: f64, t6035: f64, t2749: f64, t14866: f64, t14871: f64, t2745: f64, t4362: f64, t4364: f64, t4366: f64, t2741: f64, t5980: f64, t4365: f64, t4424: f64, t837: f64, t10770: f64, t2652: f64, t5993: f64, t14586: f64, t14786: f64, t14791: f64, t1559: f64, t4433: f64, t14785: f64, t6030: f64, t10858: f64, t6024: f64, t10816: f64, t10824: f64, t10826: f64, t6019: f64, t10698: f64, t1544: f64, t4343: f64, t5984: f64, t5988: f64, t1548: f64, t10811: f64, t6037: f64, t10846: f64, t10885: f64, t10888: f64, t10891: f64, t10900: f64, t2730: f64, t10871: f64, t836: f64, t5978: f64, t2484: f64, t10552: f64, t10554: f64, t14317: f64, t18261: f64, t18262: f64, t18265: f64, t18267: f64, t18300: f64, t18301: f64, t18308: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18378, t18390) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1194(t45, t57, t5819, t633, t5825, t80, t18281, t4186, t4328, t606, t766, t637, t83, t4335, t770, zeta_threshold);
        let (t18392, t18405) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1195(t18378, t18390, t124, t800, t828, t855, t221, t2675, t5962, t2674, t10756, t10758, t10762, t14836, t14837, t14839, t14846, t14850, t14859, t14864, t799, t851);
        let (t18411, t18416, t18420, t18424) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1196(t243, t6016, t231, t2662, t2661, t5977, t2723, t10726, t10703, t221, t5966, t2674);
        let (t18426, t18428, t18433, t18437) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1197(t125, t5977, t10786, t2747, t221, t2485, t6022, t10850, t5962, t775, t2477, t828);
        let (t18444, t18454) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1198(t14718, t6035, t2662, t2661, t125, t6016, t2747, t2749, t18426, t14866, t14871, t18411, t18416, t18420, t18424, t18428, t18433, t18437, t2745, t4362, t851);
        let (t18456, t18459, t18462, t18466, t18471, t18475) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1199(t18426, t4364, t4366, t2741, t5980, t4365, t4424, t837, t125, t5966, t10770, t2652, t5993);
        let t18489 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1200(t14586, t14786, t14791, t1559, t4433, t14785, t2652, t6030, t10858, t6024, t10816, t10824, t10826, t18456, t18459, t18462, t18466, t18471, t18475, t2745, t4362);
        let (t18491, t18495, t18500, t18507, t18511) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1201(t2741, t6019, t5966, t775, t10698, t828, t1544, t4343, t2477, t5984, t800, t5988);
        let t18524 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1202(t1548, t4343, t800, t10811, t6037, t18444, t4364, t4366, t10846, t10885, t10888, t10891, t10900, t18491, t18495, t18500, t18507, t18511, t2730, t4362, t851);
        let (t18525, t18527, t18532, t18534) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1203(t10871, t836, t18426, t4364, t221, t2485, t5978, t2484, t10552, t10554, t14317, t18261, t18262, t18265, t18267, t18300, t18301, t18308, t9278, t9308, t9316, t9329, t9333);
    (t18392, t18405, t18444, t18454, t18489, t18524, t18525, t18527, t18532, t18534)
}
