//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1334;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1335;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1336;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1337;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1338;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta376(t247: f64, t3109: f64, t4583: f64, t1063: f64, t3172: f64, t4868: f64, t1041: f64, t2862: f64, t4823: f64, t1042: f64, t1651: f64, t3181: f64, t2853: f64, t15100: f64, t15103: f64, t15377: f64, t15379: f64, t15382: f64, t15385: f64, t15388: f64, t15392: f64, t15395: f64, t15519: f64, t15522: f64, t15524: f64, t15528: f64, t15530: f64, t15536: f64, t15540: f64, t15545: f64, t3022: f64, t4729: f64, t15399: f64, t15418: f64, t15420: f64, t15423: f64, t15425: f64, t15427: f64, t15477: f64, t15515: f64, t15549: f64, t15551: f64, t15553: f64, t15555: f64, t15558: f64, t15561: f64, t15571: f64, t15575: f64, t15577: f64, t1045: f64, t373: f64, t11656: f64, t12021: f64, t16140: f64, t16144: f64, t16149: f64, t16154: f64, t1671: f64, t3124: f64, t3127: f64, t4837: f64, t4869: f64, t4875: f64, t3168: f64, t4878: f64, t13392: f64, t4801: f64, t11150: f64, t15936: f64, t4806: f64, t11144: f64, t11852: f64, t4820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16160, t16165, t16167, t16170) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1334(t247, t3109, t4583, t1063, t3172, t4868, t1041, t2862, t4823, t1042, t1651, t3181);
        let (t16172, t16179) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1335(t16170, t2853, t1042, t15100, t15103, t15377, t15379, t15382, t15385, t15388, t15392, t15395, t15519, t15522, t15524, t15528, t15530, t15536, t15540, t15545);
        let (t16181, t16182) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1336(t3022, t4729, t15399, t15418, t15420, t15423, t15425, t15427, t15477, t15515, t15549, t15551, t15553, t15555, t15558, t15561, t15571, t15575, t15577);
        let (t16183, t16189) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1337(t16179, t16182, t1045, t373, t1042, t1041, t11656, t12021, t16140, t16144, t16149, t16154, t16160, t16165, t16167, t16172, t1671, t3124, t3127, t4837, t4869, t4875);
        let (t16190, t16196, t16201, t16205, t16210, t16218) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1338(t3168, t4878, t13392, t4801, t1042, t11150, t3181, t15936, t4806, t11144, t11852, t3124, t4820);
    (t16181, t16183, t16189, t16190, t16196, t16201, t16205, t16210, t16218)
}
