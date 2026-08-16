//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta813 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2977;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2978;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2979;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2980;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2981;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta813<F: Float>(t15234: F, t3011: F, t4733: F, t981: F, t15559: F, t3022: F, t15526: F, t15525: F, t2989: F, t52647: F, t52650: F, t52652: F, t52762: F, t52806: F, t52808: F, t52923: F, t52510: F, t52516: F, t52899: F, t52905: F, t52910: F, t52912: F, t52914: F, t52916: F, t52918: F, t52920: F, t54230: F, t54231: F, t54233: F, t16052: F, t16055: F, t15752: F, t16049: F, t13392: F, t4786: F, t15599: F, t4181: F, t15968: F, t1041: F, t1042: F, t1045: F, t11268: F, t15700: F, t15701: F, t16186: F, t16226: F, t1671: F, t3124: F, t373: F, t42769: F, t42772: F, t42934: F, t4869: F, t11247: F, t1651: F, t16087: F, t53884: F, t15988: F, t3241: F, t1011: F, t15158: F, t15987: F, t11250: F, t11257: F, t11632: F, t15997: F, t16000: F, t16091: F, t3117: F, t42417: F, t42621: F, t42690: F, t42781: F, t42785: F, t43105: F, t4788: F, t4915: F, t52002: F, t15994: F, t43537: F, t53668: F, t11933: F, t16035: F, t11774: F, t127: F, t15585: F, t4872: F, t11631: F, t11662: F, t11697: F, t11703: F, t15691: F, t15696: F, t16027: F, t16089: F, t42310: F, t42695: F, t42788: F, t43082: F, t43297: F, t4573: F, t4907: F, t53670: F, t53822: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t54238, t54240, t54242, t54245, t54246) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2977::<F>(t15234, t3011, t4733, t981, t15559, t3022, t15526, t15525, t2989, t52647, t52650, t52652, t52762, t52806, t52808, t52923);
        let t54249 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2978::<F>(t52510, t52516, t52899, t52905, t52910, t52912, t52914, t52916, t52918, t52920, t54230, t54231, t54233, t54246);
        let (t54263, t54267, t54271, t54275) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2979::<F>(t16052, t16055, t15752, t16049, t13392, t4786, t15599, t4181, t15968, t1041, t1042, t1045, t11268, t15700, t15701, t16186, t16226, t1671, t3124, t373, t42769, t42772, t42934, t4869, t54249);
        let (t54276, t54308) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2980::<F>(t11247, t1651, t16087, t53884, t15988, t3241, t1011, t15158, t15987, t11250, t11257, t11632, t15997, t16000, t16091, t3117, t42417, t42621, t42690, t42781, t42785, t43105, t4788, t4915, t52002);
        let t54346 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2981::<F>(t15994, t3241, t43537, t53668, t11933, t16035, t11774, t127, t15585, t4872, t11247, t11631, t11662, t11697, t11703, t15691, t15696, t15700, t16027, t16089, t3117, t42310, t42695, t42788, t43082, t43297, t4573, t4907, t53670, t53822);
    (t54238, t54240, t54242, t54245, t54249, t54263, t54267, t54271, t54275, t54276, t54308, t54346)
}
