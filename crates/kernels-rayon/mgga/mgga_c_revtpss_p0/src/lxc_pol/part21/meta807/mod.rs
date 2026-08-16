//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta807 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2943;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2944;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2945;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta807(t1011: f64, t4886: f64, t697: f64, t1065: f64, t372: f64, t4866: f64, t11774: f64, t16103: f64, t42254: f64, t42257: f64, t42259: f64, t42268: f64, t42270: f64, t42274: f64, t42282: f64, t42284: f64, t42288: f64, t11670: f64, t15904: f64, t12167: f64, t11922: f64, t16081: f64, t16083: f64, t11675: f64, t15682: f64, t11711: f64, t15618: f64, t11667: f64, t11696: f64, t11703: f64, t11705: f64, t11866: f64, t15697: f64, t15917: f64, t15957: f64, t16022: f64, t16045: f64, t16084: f64, t19741: f64, t3091: f64, t3092: f64, t42397: f64, t43066: f64, t4781: f64, t1043: f64, t1469: f64, t3133: f64, t3162: f64, t3115: f64, t42793: f64, t4906: f64, t1045: f64, t15584: f64, t15689: f64, t15691: f64, t16226: f64, t16227: f64, t19980: f64, t2251: f64, t2258: f64, t2852: f64, t3075: f64, t3155: f64, t42324: f64, t42326: f64, t42334: f64, t42336: f64, t42338: f64, t43301: f64, t606: f64, t905: f64, t999: f64, t11722: f64, t4834: f64, t11727: f64, t16143: f64, t3127: f64, t3172: f64, t15772: f64, t3106: f64, t15775: f64, t1042: f64, t11160: f64, t15611: f64, t15725: f64, t15728: f64, t15839: f64, t15893: f64, t16149: f64, t3117: f64, t42346: f64, t42643: f64, t43044: f64, t4823: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t53545, t53549) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2943(t1011, t4886, t697, t1065, t372, t4866, t11774, t16103, t42254, t42257, t42259, t42268, t42270, t42274, t42282, t42284, t42288);
        let (t53552, t53581) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2944(t11670, t15904, t12167, t11922, t16081, t16083, t11675, t15682, t11711, t15618, t11667, t11696, t11703, t11705, t11866, t15697, t15917, t15957, t16022, t16045, t16084, t19741, t3091, t3092, t42397, t43066, t4781);
        let (t53585, t53617) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2945(t1043, t1469, t3133, t3162, t3115, t42793, t4906, t1045, t11774, t15584, t15689, t15691, t16226, t16227, t19980, t2251, t2258, t2852, t3075, t3155, t42324, t42326, t42334, t42336, t42338, t43301, t606, t905);
        let t53645 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2946(t3162, t999, t11722, t4834, t11727, t16143, t3127, t3172, t15772, t3106, t15775, t1042, t11160, t15611, t15725, t15728, t15839, t15893, t16149, t3117, t42346, t42643, t43044, t4823);
    (t53545, t53549, t53552, t53581, t53585, t53617, t53645)
}
