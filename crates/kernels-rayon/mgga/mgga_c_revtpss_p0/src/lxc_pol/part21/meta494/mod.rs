//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2085;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2086;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2087;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta494(t15604: f64, t4893: f64, t3117: f64, t4894: f64, t999: f64, t4583: f64, t4786: f64, t3092: f64, t3090: f64, t4954: f64, t4757: f64, t11264: f64, t11271: f64, t11774: f64, t11859: f64, t11875: f64, t11927: f64, t15583: f64, t15586: f64, t15592: f64, t15596: f64, t15601: f64, t3091: f64, t3097: f64, t15125: f64, t15191: f64, t11133: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t15127: f64, t15132: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15189: f64, t15195: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15605, t15606, t15609, t15610, t15611, t15614, t15615, t15618) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2085(t15604, t4893, t3117, t4894, t999, t4583, t4786, t3092, t3090, t4954);
        let (t15621, t15622, t15625) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2086(t4757, t4786, t3117, t11264, t11271, t11774, t11859, t11875, t11927, t15583, t15586, t15592, t15596, t15601, t15606, t15611, t15615, t15618, t3091, t3097);
        let t15648 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2087(t15125, t15191, t11133, t11134, t11136, t11138, t11140, t15127, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
    (t15605, t15606, t15609, t15610, t15611, t15614, t15615, t15618, t15621, t15622, t15625, t15648)
}
