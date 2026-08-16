//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta805 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2928;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2929;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2930;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2931;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2932;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta805(t53251: f64, t53272: f64, t11223: f64, t1678: f64, t16163: f64, t3169: f64, t1041: f64, t11262: f64, t4868: f64, t1058: f64, t15859: f64, t3201: f64, t4794: f64, t15866: f64, t15888: f64, t11656: f64, t11961: f64, t12004: f64, t15811: f64, t1659: f64, t225: f64, t366: f64, t375: f64, t4803: f64, t4808: f64, t52977: f64, t4798: f64, t343: f64, t44: f64, t816: f64, t11821: f64, t65: f64, t11144: f64, t11970: f64, t1660: f64, t27527: f64, t2852: f64, t11150: f64, t27531: f64, t15908: f64, t999: f64, t1042: f64, t1053: f64, t11804: f64, t15716: f64, t15887: f64, t15907: f64, t1663: f64, t247: f64, t3116: f64, t3117: f64, t3230: f64, t42967: f64, t43105: f64, t4788: f64, t4797: f64, t4837: f64, t51959: f64, t53192: f64, t15817: f64, t3173: f64, t16158: f64, t3188: f64, t1063: f64, t15193: f64, t3109: f64, t11233: f64, t12026: f64, t15707: f64, t15791: f64, t15830: f64, t15834: f64, t15952: f64, t3106: f64, t3177: f64, t3184: f64, t42391: f64, t4825: f64, t4834: f64, t11710: f64, t15600: f64, t3091: f64, t127: f64, t4823: f64, t11774: f64, t3096: f64, t11675: f64, t15592: f64, t15596: f64, t42121: f64, t42122: f64, t42124: f64, t42139: f64, t42141: f64, t42146: f64, t42149: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t53273, t53281, t53290, t53294, t53298, t53300) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2928(t53251, t53272, t11223, t1678, t16163, t3169, t1041, t11262, t4868, t1058, t15859, t3201, t4794);
        let t53310 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2929(t1058, t15866, t15888, t11656, t11961, t12004, t15811, t1659, t225, t366, t375, t4803, t4808, t52977, t53290, t53294, t53298, t53300);
        let (t53318, t53320, t53322, t53326, t53328) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2930(t3201, t4798, t343, t44, t816, t11821, t65, t11144, t11970, t1660, t27527, t2852);
        let (t53340, t53351) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2931(t11150, t27531, t15908, t999, t1042, t1053, t11804, t15716, t15887, t15907, t1663, t247, t3116, t3117, t3230, t375, t42967, t43105, t4788, t4797, t4837, t51959, t53192, t53318, t53320, t53322, t53326, t53328);
        let t53377 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2932(t15817, t3173, t16158, t3188, t1063, t15193, t247, t3109, t11233, t11656, t12026, t15707, t15791, t15830, t15834, t15952, t3106, t3177, t3184, t42391, t4825, t4834);
        let t53395 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2933(t11710, t15600, t3091, t127, t4823, t11774, t3096, t11675, t15592, t15596, t42121, t42122, t42124, t42139, t42141, t42146, t42149);
    (t53273, t53281, t53310, t53340, t53351, t53377, t53395)
}
