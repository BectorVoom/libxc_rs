//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta817 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3005;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3006;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3007;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3008;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3009;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta817<F: Float>(t11710: F, t15974: F, t4899: F, t16183: F, t3153: F, t11866: F, t15794: F, t11671: F, t15925: F, t15752: F, t15917: F, t127: F, t15700: F, t15702: F, t4801: F, t1043: F, t43116: F, t3075: F, t4900: F, t1063: F, t11986: F, t247: F, t4583: F, t11859: F, t11875: F, t15609: F, t15703: F, t15780: F, t3117: F, t3120: F, t4893: F, t53923: F, t3133: F, t3155: F, t11173: F, t1651: F, t1042: F, t11675: F, t11845: F, t11855: F, t12004: F, t1469: F, t15584: F, t15615: F, t16040: F, t16222: F, t16226: F, t3116: F, t3127: F, t43063: F, t43244: F, t4783: F, t4831: F, t4834: F, t4837: F, t4872: F, t53585: F, t54271: F, t1062: F, t43154: F, t11202: F, t11940: F, t3105: F, t11923: F, t15926: F, t11922: F, t16016: F, t11994: F, t15734: F, t15830: F, t3111: F, t16035: F, t16088: F, t342: F, t380: F, t11231: F, t11703: F, t11748: F, t15153: F, t15719: F, t15837: F, t16089: F, t19705: F, t3092: F, t53835: F, t906: F, t16219: F, t3241: F, t11637: F, t11672: F, t15850: F, t15965: F, t16027: F, t16095: F, t3184: F, t43129: F, t43133: F, t43146: F, t43169: F, t43285: F, t43512: F, t43611: F, t4891: F, t4896: F, t4902: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t54907, t54909, t54914, t54916, t54919, t54925) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3005::<F>(t11710, t15974, t4899, t16183, t3153, t11866, t15794, t11671, t15925, t15752, t15917, t127, t15700, t15702, t4801);
        let (t54931, t54936, t54945) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3006::<F>(t1043, t43116, t3075, t4900, t1063, t11986, t247, t4583, t11859, t11875, t15609, t15703, t15780, t3117, t3120, t4893, t4899, t53923, t54907, t54909, t54914, t54916, t54919, t54925);
        let (t54955, t54977) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3007::<F>(t3133, t3155, t11173, t1651, t1042, t11675, t11845, t11855, t11866, t12004, t1469, t15584, t15615, t16040, t16222, t16226, t247, t3116, t3127, t43063, t43244, t4783, t4831, t4834, t4837, t4872, t53585, t54271);
        let (t54982, t54983, t54988, t54991, t54994, t55000) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3008::<F>(t1062, t43154, t11202, t1651, t11940, t3105, t11923, t15926, t11922, t16016, t4899, t11994, t15734);
        let (t55011, t55016) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3009::<F>(t15830, t3111, t11866, t16035, t16088, t342, t380, t11231, t11703, t11748, t15153, t15719, t15837, t16089, t19705, t247, t3092, t3116, t4834, t53835, t54982, t54983, t54988, t54991, t54994, t55000, t906);
        let t55039 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3010::<F>(t16219, t3241, t11637, t11672, t11703, t15153, t15850, t15965, t16027, t16095, t3184, t43129, t43133, t43146, t43169, t43285, t43512, t43611, t4891, t4896, t4902);
    (t54909, t54931, t54936, t54945, t54955, t54977, t54983, t55011, t55016, t55039)
}
