//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2085;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2086;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2087;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta494<F: Float>(t15604: F, t4893: F, t3117: F, t4894: F, t999: F, t4583: F, t4786: F, t3092: F, t3090: F, t4954: F, t4757: F, t11264: F, t11271: F, t11774: F, t11859: F, t11875: F, t11927: F, t15583: F, t15586: F, t15592: F, t15596: F, t15601: F, t3091: F, t3097: F, t15125: F, t15191: F, t11133: F, t11134: F, t11136: F, t11138: F, t11140: F, t15127: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t15195: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15605, t15606, t15609, t15610, t15611, t15614, t15615, t15618) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2085::<F>(t15604, t4893, t3117, t4894, t999, t4583, t4786, t3092, t3090, t4954);
        let (t15621, t15622, t15625) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2086::<F>(t4757, t4786, t3117, t11264, t11271, t11774, t11859, t11875, t11927, t15583, t15586, t15592, t15596, t15601, t15606, t15611, t15615, t15618, t3091, t3097);
        let t15648 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2087::<F>(t15125, t15191, t11133, t11134, t11136, t11138, t11140, t15127, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
    (t15605, t15606, t15609, t15610, t15611, t15614, t15615, t15618, t15621, t15622, t15625, t15648)
}
