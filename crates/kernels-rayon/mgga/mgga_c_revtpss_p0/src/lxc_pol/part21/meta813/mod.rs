//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta813 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2977;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2978;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2979;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2980;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2981;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta813(t15234: f64, t3011: f64, t4733: f64, t981: f64, t15559: f64, t3022: f64, t15526: f64, t15525: f64, t2989: f64, t52647: f64, t52650: f64, t52652: f64, t52762: f64, t52806: f64, t52808: f64, t52923: f64, t52510: f64, t52516: f64, t52899: f64, t52905: f64, t52910: f64, t52912: f64, t52914: f64, t52916: f64, t52918: f64, t52920: f64, t54230: f64, t54231: f64, t54233: f64, t16052: f64, t16055: f64, t15752: f64, t16049: f64, t13392: f64, t4786: f64, t15599: f64, t4181: f64, t15968: f64, t1041: f64, t1042: f64, t1045: f64, t11268: f64, t15700: f64, t15701: f64, t16186: f64, t16226: f64, t1671: f64, t3124: f64, t373: f64, t42769: f64, t42772: f64, t42934: f64, t4869: f64, t11247: f64, t1651: f64, t16087: f64, t53884: f64, t15988: f64, t3241: f64, t1011: f64, t15158: f64, t15987: f64, t11250: f64, t11257: f64, t11632: f64, t15997: f64, t16000: f64, t16091: f64, t3117: f64, t42417: f64, t42621: f64, t42690: f64, t42781: f64, t42785: f64, t43105: f64, t4788: f64, t4915: f64, t52002: f64, t15994: f64, t43537: f64, t53668: f64, t11933: f64, t16035: f64, t11774: f64, t127: f64, t15585: f64, t4872: f64, t11631: f64, t11662: f64, t11697: f64, t11703: f64, t15691: f64, t15696: f64, t16027: f64, t16089: f64, t42310: f64, t42695: f64, t42788: f64, t43082: f64, t43297: f64, t4573: f64, t4907: f64, t53670: f64, t53822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54238, t54240, t54242, t54245, t54246) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2977(t15234, t3011, t4733, t981, t15559, t3022, t15526, t15525, t2989, t52647, t52650, t52652, t52762, t52806, t52808, t52923);
        let t54249 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2978(t52510, t52516, t52899, t52905, t52910, t52912, t52914, t52916, t52918, t52920, t54230, t54231, t54233, t54246);
        let (t54263, t54267, t54271, t54275) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2979(t16052, t16055, t15752, t16049, t13392, t4786, t15599, t4181, t15968, t1041, t1042, t1045, t11268, t15700, t15701, t16186, t16226, t1671, t3124, t373, t42769, t42772, t42934, t4869, t54249);
        let (t54276, t54308) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2980(t11247, t1651, t16087, t53884, t15988, t3241, t1011, t15158, t15987, t11250, t11257, t11632, t15997, t16000, t16091, t3117, t42417, t42621, t42690, t42781, t42785, t43105, t4788, t4915, t52002);
        let t54346 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2981(t15994, t3241, t43537, t53668, t11933, t16035, t11774, t127, t15585, t4872, t11247, t11631, t11662, t11697, t11703, t15691, t15696, t15700, t16027, t16089, t3117, t42310, t42695, t42788, t43082, t43297, t4573, t4907, t53670, t53822);
    (t54238, t54240, t54242, t54245, t54249, t54263, t54267, t54271, t54275, t54276, t54308, t54346)
}
