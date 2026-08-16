//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta817 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3005;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3006;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3007;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3008;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3009;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta817(t11710: f64, t15974: f64, t4899: f64, t16183: f64, t3153: f64, t11866: f64, t15794: f64, t11671: f64, t15925: f64, t15752: f64, t15917: f64, t127: f64, t15700: f64, t15702: f64, t4801: f64, t1043: f64, t43116: f64, t3075: f64, t4900: f64, t1063: f64, t11986: f64, t247: f64, t4583: f64, t11859: f64, t11875: f64, t15609: f64, t15703: f64, t15780: f64, t3117: f64, t3120: f64, t4893: f64, t53923: f64, t3133: f64, t3155: f64, t11173: f64, t1651: f64, t1042: f64, t11675: f64, t11845: f64, t11855: f64, t12004: f64, t1469: f64, t15584: f64, t15615: f64, t16040: f64, t16222: f64, t16226: f64, t3116: f64, t3127: f64, t43063: f64, t43244: f64, t4783: f64, t4831: f64, t4834: f64, t4837: f64, t4872: f64, t53585: f64, t54271: f64, t1062: f64, t43154: f64, t11202: f64, t11940: f64, t3105: f64, t11923: f64, t15926: f64, t11922: f64, t16016: f64, t11994: f64, t15734: f64, t15830: f64, t3111: f64, t16035: f64, t16088: f64, t342: f64, t380: f64, t11231: f64, t11703: f64, t11748: f64, t15153: f64, t15719: f64, t15837: f64, t16089: f64, t19705: f64, t3092: f64, t53835: f64, t906: f64, t16219: f64, t3241: f64, t11637: f64, t11672: f64, t15850: f64, t15965: f64, t16027: f64, t16095: f64, t3184: f64, t43129: f64, t43133: f64, t43146: f64, t43169: f64, t43285: f64, t43512: f64, t43611: f64, t4891: f64, t4896: f64, t4902: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54907, t54909, t54914, t54916, t54919, t54925) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3005(t11710, t15974, t4899, t16183, t3153, t11866, t15794, t11671, t15925, t15752, t15917, t127, t15700, t15702, t4801);
        let (t54931, t54936, t54945) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3006(t1043, t43116, t3075, t4900, t1063, t11986, t247, t4583, t11859, t11875, t15609, t15703, t15780, t3117, t3120, t4893, t4899, t53923, t54907, t54909, t54914, t54916, t54919, t54925);
        let (t54955, t54977) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3007(t3133, t3155, t11173, t1651, t1042, t11675, t11845, t11855, t11866, t12004, t1469, t15584, t15615, t16040, t16222, t16226, t247, t3116, t3127, t43063, t43244, t4783, t4831, t4834, t4837, t4872, t53585, t54271);
        let (t54982, t54983, t54988, t54991, t54994, t55000) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3008(t1062, t43154, t11202, t1651, t11940, t3105, t11923, t15926, t11922, t16016, t4899, t11994, t15734);
        let (t55011, t55016) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3009(t15830, t3111, t11866, t16035, t16088, t342, t380, t11231, t11703, t11748, t15153, t15719, t15837, t16089, t19705, t247, t3092, t3116, t4834, t53835, t54982, t54983, t54988, t54991, t54994, t55000, t906);
        let t55039 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3010(t16219, t3241, t11637, t11672, t11703, t15153, t15850, t15965, t16027, t16095, t3184, t43129, t43133, t43146, t43169, t43285, t43512, t43611, t4891, t4896, t4902);
    (t54909, t54931, t54936, t54945, t54955, t54977, t54983, t55011, t55016, t55039)
}
