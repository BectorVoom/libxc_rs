//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta408 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1476;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1477;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1478;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1479;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1480;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta408<F: Float>(t11705: F, t4781: F, t11703: F, t11678: F, t357: F, t1592: F, t3092: F, t4900: F, t999: F, t4893: F, t3117: F, t4894: F, t4583: F, t4786: F, t3090: F, t4954: F, t4757: F, t11264: F, t11271: F, t11774: F, t11859: F, t11875: F, t11927: F, t15583: F, t15586: F, t15592: F, t3091: F, t3097: F, t15125: F, t15191: F, t11133: F, t11134: F, t11136: F, t11138: F, t11140: F, t15127: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t15195: F, t373: F, t371: F, t372: F, t4742: F, t993: F, t225: F, t366: F, t3224: F, t4845: F, t127: F, t4852: F, t1025: F, t1646: F, t3056: F, t3106: F, t4817: F, t1028: F, t11644: F, t11649: F, t11783: F, t1665: F, t3208: F, t3211: F, t3220: F, t4854: F, t4858: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15596, t15601, t15604, t15606, t15609) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1476::<F>(t11705, t4781, t11703, t11678, t357, t1592, t3092, t4900, t999, t4893, t3117, t4894);
        let t15625 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1477::<F>(t15609, t4893, t3117, t4583, t4786, t3092, t3090, t4954, t4757, t11264, t11271, t11774, t11859, t11875, t11927, t15583, t15586, t15592, t15596, t15601, t15606, t3091, t3097);
        let t15648 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1478::<F>(t15125, t15191, t11133, t11134, t11136, t11138, t11140, t15127, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
        let (t15651, t15654, t15655, t15656, t15662, t15666) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1479::<F>(t15648, t373, t371, t372, t4742, t993, t225, t366, t3224, t4845, t127, t4852);
        let (t15669, t15670, t15676) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1480::<F>(t1025, t15666, t1646, t3056, t225, t366, t3106, t4817, t1028, t11644, t11649, t11783, t15651, t15656, t15662, t1665, t3208, t3211, t3220, t3224, t4854, t4858);
    (t15604, t15609, t15625, t15648, t15654, t15655, t15669, t15670, t15676)
}
