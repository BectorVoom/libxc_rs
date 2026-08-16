//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1476;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1477;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1478;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1479;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1480;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta408(t11705: f64, t4781: f64, t11703: f64, t11678: f64, t357: f64, t1592: f64, t3092: f64, t4900: f64, t999: f64, t4893: f64, t3117: f64, t4894: f64, t4583: f64, t4786: f64, t3090: f64, t4954: f64, t4757: f64, t11264: f64, t11271: f64, t11774: f64, t11859: f64, t11875: f64, t11927: f64, t15583: f64, t15586: f64, t15592: f64, t3091: f64, t3097: f64, t15125: f64, t15191: f64, t11133: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t15127: f64, t15132: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15189: f64, t15195: f64, t373: f64, t371: f64, t372: f64, t4742: f64, t993: f64, t225: f64, t366: f64, t3224: f64, t4845: f64, t127: f64, t4852: f64, t1025: f64, t1646: f64, t3056: f64, t3106: f64, t4817: f64, t1028: f64, t11644: f64, t11649: f64, t11783: f64, t1665: f64, t3208: f64, t3211: f64, t3220: f64, t4854: f64, t4858: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15596, t15601, t15604, t15606, t15609) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1476(t11705, t4781, t11703, t11678, t357, t1592, t3092, t4900, t999, t4893, t3117, t4894);
        let t15625 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1477(t15609, t4893, t3117, t4583, t4786, t3092, t3090, t4954, t4757, t11264, t11271, t11774, t11859, t11875, t11927, t15583, t15586, t15592, t15596, t15601, t15606, t3091, t3097);
        let t15648 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1478(t15125, t15191, t11133, t11134, t11136, t11138, t11140, t15127, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
        let (t15651, t15654, t15655, t15656, t15662, t15666) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1479(t15648, t373, t371, t372, t4742, t993, t225, t366, t3224, t4845, t127, t4852);
        let (t15669, t15670, t15676) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1480(t1025, t15666, t1646, t3056, t225, t366, t3106, t4817, t1028, t11644, t11649, t11783, t15651, t15656, t15662, t1665, t3208, t3211, t3220, t3224, t4854, t4858);
    (t15604, t15609, t15625, t15648, t15654, t15655, t15669, t15670, t15676)
}
