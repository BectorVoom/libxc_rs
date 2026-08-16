//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta414 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1511;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1512;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1513;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1514;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1515;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta414<F: Float>(t247: F, t3109: F, t4583: F, t1063: F, t3172: F, t4868: F, t1041: F, t2862: F, t4823: F, t1042: F, t1651: F, t3181: F, t2853: F, t15100: F, t15103: F, t15377: F, t15379: F, t15382: F, t15385: F, t15388: F, t15392: F, t15395: F, t15519: F, t15522: F, t15524: F, t15528: F, t15530: F, t15536: F, t15540: F, t15545: F, t3022: F, t4729: F, t15399: F, t15418: F, t15420: F, t15423: F, t15425: F, t15427: F, t15477: F, t15515: F, t15549: F, t15551: F, t15553: F, t15555: F, t15558: F, t15561: F, t15571: F, t15575: F, t15577: F, t1045: F, t373: F, t11656: F, t12021: F, t16140: F, t16144: F, t16149: F, t16154: F, t1671: F, t3124: F, t3127: F, t4837: F, t4869: F, t4875: F, t3168: F, t4878: F, t13392: F, t4801: F, t11150: F, t15936: F, t4806: F, t11144: F, t11852: F, t4820: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16160, t16165, t16167, t16170) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1511::<F>(t247, t3109, t4583, t1063, t3172, t4868, t1041, t2862, t4823, t1042, t1651, t3181);
        let (t16172, t16179) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1512::<F>(t16170, t2853, t1042, t15100, t15103, t15377, t15379, t15382, t15385, t15388, t15392, t15395, t15519, t15522, t15524, t15528, t15530, t15536, t15540, t15545);
        let (t16181, t16182) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1513::<F>(t3022, t4729, t15399, t15418, t15420, t15423, t15425, t15427, t15477, t15515, t15549, t15551, t15553, t15555, t15558, t15561, t15571, t15575, t15577);
        let (t16183, t16189) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1514::<F>(t16179, t16182, t1045, t373, t1042, t1041, t11656, t12021, t16140, t16144, t16149, t16154, t16160, t16165, t16167, t16172, t1671, t3124, t3127, t4837, t4869, t4875);
        let (t16190, t16196, t16201, t16205, t16210, t16218) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1515::<F>(t3168, t4878, t13392, t4801, t1042, t11150, t3181, t15936, t4806, t11144, t11852, t3124, t4820);
    (t16181, t16183, t16189, t16190, t16196, t16201, t16205, t16210, t16218)
}
