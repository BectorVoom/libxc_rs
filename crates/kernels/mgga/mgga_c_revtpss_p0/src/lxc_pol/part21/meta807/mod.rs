//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta807 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2943;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2944;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2945;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta807<F: Float>(t1011: F, t4886: F, t697: F, t1065: F, t372: F, t4866: F, t11774: F, t16103: F, t42254: F, t42257: F, t42259: F, t42268: F, t42270: F, t42274: F, t42282: F, t42284: F, t42288: F, t11670: F, t15904: F, t12167: F, t11922: F, t16081: F, t16083: F, t11675: F, t15682: F, t11711: F, t15618: F, t11667: F, t11696: F, t11703: F, t11705: F, t11866: F, t15697: F, t15917: F, t15957: F, t16022: F, t16045: F, t16084: F, t19741: F, t3091: F, t3092: F, t42397: F, t43066: F, t4781: F, t1043: F, t1469: F, t3133: F, t3162: F, t3115: F, t42793: F, t4906: F, t1045: F, t15584: F, t15689: F, t15691: F, t16226: F, t16227: F, t19980: F, t2251: F, t2258: F, t2852: F, t3075: F, t3155: F, t42324: F, t42326: F, t42334: F, t42336: F, t42338: F, t43301: F, t606: F, t905: F, t999: F, t11722: F, t4834: F, t11727: F, t16143: F, t3127: F, t3172: F, t15772: F, t3106: F, t15775: F, t1042: F, t11160: F, t15611: F, t15725: F, t15728: F, t15839: F, t15893: F, t16149: F, t3117: F, t42346: F, t42643: F, t43044: F, t4823: F) -> (F, F, F, F, F, F, F) {
        let (t53545, t53549) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2943::<F>(t1011, t4886, t697, t1065, t372, t4866, t11774, t16103, t42254, t42257, t42259, t42268, t42270, t42274, t42282, t42284, t42288);
        let (t53552, t53581) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2944::<F>(t11670, t15904, t12167, t11922, t16081, t16083, t11675, t15682, t11711, t15618, t11667, t11696, t11703, t11705, t11866, t15697, t15917, t15957, t16022, t16045, t16084, t19741, t3091, t3092, t42397, t43066, t4781);
        let (t53585, t53617) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2945::<F>(t1043, t1469, t3133, t3162, t3115, t42793, t4906, t1045, t11774, t15584, t15689, t15691, t16226, t16227, t19980, t2251, t2258, t2852, t3075, t3155, t42324, t42326, t42334, t42336, t42338, t43301, t606, t905);
        let t53645 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2946::<F>(t3162, t999, t11722, t4834, t11727, t16143, t3127, t3172, t15772, t3106, t15775, t1042, t11160, t15611, t15725, t15728, t15839, t15893, t16149, t3117, t42346, t42643, t43044, t4823);
    (t53545, t53549, t53552, t53581, t53585, t53617, t53645)
}
