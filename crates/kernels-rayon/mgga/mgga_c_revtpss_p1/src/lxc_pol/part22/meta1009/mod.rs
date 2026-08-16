//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1009 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3452;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3453;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3454;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3455;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3456;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3457;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3458;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3459;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3460;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3461;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3462;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1009(t41361: f64, t41363: f64, t51973: f64, t51978: f64, t63325: f64, t63328: f64, t63336: f64, t63338: f64, t63340: f64, t63342: f64, t63346: f64, t63351: f64, t63355: f64, t52033: f64, t52035: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64, t63359: f64, t63361: f64, t63366: f64, t63369: f64, t63371: f64, t63374: f64, t41330: f64, t41332: f64, t52047: f64, t52049: f64, t52051: f64, t63399: f64, t63447: f64, t63451: f64, t63453: f64, t63457: f64, t63459: f64, t63462: f64, t63464: f64, t64945: f64, t1024: f64, t1043: f64, t1082: f64, t1083: f64, t1089: f64, t11788: f64, t12149: f64, t16414: f64, t16436: f64, t16458: f64, t1651: f64, t1685: f64, t1689: f64, t19380: f64, t19414: f64, t19479: f64, t19515: f64, t3204: f64, t3278: f64, t3291: f64, t3298: f64, t342: f64, t4743: f64, t4930: f64, t4954: f64, t4984: f64, t5012: f64, t53865: f64, t55649: f64, t55747: f64, t55868: f64, t55991: f64, t64907: f64, t64912: f64, t64916: f64, t42013: f64, t51967: f64, t63274: f64, t63276: f64, t63278: f64, t63281: f64, t63285: f64, t63290: f64, t63293: f64, t63299: f64, t63304: f64, t63308: f64, t341: f64, t15648: f64, t1668: f64, t3059: f64, t6258: f64, t3057: f64, t3140: f64, t1035: f64, t1093: f64, t11940: f64, t12146: f64, t15670: f64, t15837: f64, t16402: f64, t16488: f64, t16584: f64, t19453: f64, t19584: f64, t19856: f64, t20136: f64, t3151: f64, t3287: f64, t357: f64, t378: f64, t381: f64, t43350: f64, t4961: f64, t4977: f64, t4980: f64, t4995: f64, t4999: f64, t5004: f64, t54695: f64, t55685: f64, t55805: f64, t55948: f64, t64891: f64, t4772: f64, t11249: f64, t6299: f64, t12073: f64, t16152: f64, t16183: f64, t16399: f64, t16433: f64, t16449: f64, t16485: f64, t16496: f64, t16566: f64, t16568: f64, t19556: f64, t19580: f64, t19617: f64, t3143: f64, t43154: f64, t43453: f64, t4893: f64, t4981: f64, t4982: f64, t55330: f64, t55701: f64, t55764: f64, t6244: f64, t64647: f64, t64772: f64, t1647: f64, t16565: f64, t3133: f64, t12127: f64, t16520: f64, t16537: f64, t16540: f64, t16552: f64, t16569: f64, t16573: f64, t16577: f64, t16581: f64, t19446: f64, t19450: f64, t19501: f64, t19521: f64, t19526: f64, t19593: f64, t20133: f64, t3223: f64, t43438: f64, t43446: f64, t43512: f64, t43568: f64, t4964: f64, t55646: f64, t55732: f64, t56049: f64, t6375: f64, t64835: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t64959 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3452(t41361, t41363, t51973, t51978, t63325, t63328, t63336, t63338, t63340, t63342, t63346, t63351, t63355);
        let t64973 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3453(t52033, t52035, t52037, t52039, t52041, t52045, t63359, t63361, t63366, t63369, t63371, t63374);
        let t64987 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3454(t41330, t41332, t52047, t52049, t52051, t63399, t63447, t63451, t63453, t63457, t63459, t63462, t63464);
        let (t64989, t64997) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3455(t64945, t64959, t64973, t64987, t1024, t1043, t1082, t1083, t1089, t11788, t12149, t16414, t16436, t16458, t1651, t1685, t1689, t19380, t19414, t19479, t19515, t3204, t3278, t3291, t3298, t342, t4743, t4930, t4954, t4984, t5012, t53865, t55649, t55747, t55868, t55991, t64907, t64912, t64916);
        let t65012 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3456(t42013, t51967, t63274, t63276, t63278, t63281, t63285, t63290, t63293, t63299, t63304, t63308);
        let t65026 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3457(t41361, t41363, t51973, t51978, t63325, t63328, t63336, t63338, t63340, t63342, t63346, t63351, t63355);
        let t65040 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3458(t52033, t52035, t52037, t52039, t52041, t52045, t63359, t63361, t63366, t63369, t63371, t63374);
        let t65054 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3459(t41330, t41332, t52047, t52049, t52051, t63399, t63447, t63451, t63453, t63457, t63459, t63462, t63464);
        let (t65057, t65060, t65071, t65096, t65102) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3460(t341, t65012, t65026, t65040, t65054, t15648, t1668, t3059, t6258, t3057, t3140, t1035, t1082, t1089, t1093, t11940, t12146, t15670, t15837, t16402, t16436, t16488, t1651, t16584, t19453, t19584, t19856, t20136, t3151, t3204, t3278, t3287, t357, t378, t381, t43350, t4743, t4961, t4977, t4980, t4984, t4995, t4999, t5004, t54695, t55685, t55805, t55948, t64891);
        let (t65122, t65144, t65150) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3461(t4772, t11249, t6299, t1024, t1082, t11788, t11940, t12073, t15670, t16152, t16183, t16399, t16433, t16449, t16485, t16496, t1651, t16566, t16568, t19556, t19580, t19617, t3059, t3143, t3204, t378, t43154, t43453, t4893, t4954, t4977, t4981, t4982, t55330, t55701, t55764, t6244, t64647, t64772, t65096);
        let (t65186, t65192, t65196) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3462(t1647, t16565, t3133, t6258, t16183, t1651, t1082, t1089, t12127, t16520, t16537, t16540, t16552, t16569, t16573, t16577, t16581, t19446, t19450, t19501, t19521, t19526, t19593, t20133, t3059, t3204, t3223, t3287, t43438, t43446, t43512, t43568, t4964, t4982, t55646, t55685, t55732, t56049, t6375, t64835);
    (t64989, t64997, t65057, t65060, t65071, t65102, t65122, t65144, t65150, t65186, t65192, t65196)
}
