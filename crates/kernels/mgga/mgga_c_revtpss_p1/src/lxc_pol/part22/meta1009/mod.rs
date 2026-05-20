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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1009<F: Float>(t41361: F, t41363: F, t51973: F, t51978: F, t63325: F, t63328: F, t63336: F, t63338: F, t63340: F, t63342: F, t63346: F, t63351: F, t63355: F, t52033: F, t52035: F, t52037: F, t52039: F, t52041: F, t52045: F, t63359: F, t63361: F, t63366: F, t63369: F, t63371: F, t63374: F, t41330: F, t41332: F, t52047: F, t52049: F, t52051: F, t63399: F, t63447: F, t63451: F, t63453: F, t63457: F, t63459: F, t63462: F, t63464: F, t64945: F, t1024: F, t1043: F, t1082: F, t1083: F, t1089: F, t11788: F, t12149: F, t16414: F, t16436: F, t16458: F, t1651: F, t1685: F, t1689: F, t19380: F, t19414: F, t19479: F, t19515: F, t3204: F, t3278: F, t3291: F, t3298: F, t342: F, t4743: F, t4930: F, t4954: F, t4984: F, t5012: F, t53865: F, t55649: F, t55747: F, t55868: F, t55991: F, t64907: F, t64912: F, t64916: F, t42013: F, t51967: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F, t63299: F, t63304: F, t63308: F, t341: F, t15648: F, t1668: F, t3059: F, t6258: F, t3057: F, t3140: F, t1035: F, t1093: F, t11940: F, t12146: F, t15670: F, t15837: F, t16402: F, t16488: F, t16584: F, t19453: F, t19584: F, t19856: F, t20136: F, t3151: F, t3287: F, t357: F, t378: F, t381: F, t43350: F, t4961: F, t4977: F, t4980: F, t4995: F, t4999: F, t5004: F, t54695: F, t55685: F, t55805: F, t55948: F, t64891: F, t4772: F, t11249: F, t6299: F, t12073: F, t16152: F, t16183: F, t16399: F, t16433: F, t16449: F, t16485: F, t16496: F, t16566: F, t16568: F, t19556: F, t19580: F, t19617: F, t3143: F, t43154: F, t43453: F, t4893: F, t4981: F, t4982: F, t55330: F, t55701: F, t55764: F, t6244: F, t64647: F, t64772: F, t1647: F, t16565: F, t3133: F, t12127: F, t16520: F, t16537: F, t16540: F, t16552: F, t16569: F, t16573: F, t16577: F, t16581: F, t19446: F, t19450: F, t19501: F, t19521: F, t19526: F, t19593: F, t20133: F, t3223: F, t43438: F, t43446: F, t43512: F, t43568: F, t4964: F, t55646: F, t55732: F, t56049: F, t6375: F, t64835: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t64959 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3452::<F>(t41361, t41363, t51973, t51978, t63325, t63328, t63336, t63338, t63340, t63342, t63346, t63351, t63355);
        let t64973 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3453::<F>(t52033, t52035, t52037, t52039, t52041, t52045, t63359, t63361, t63366, t63369, t63371, t63374);
        let t64987 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3454::<F>(t41330, t41332, t52047, t52049, t52051, t63399, t63447, t63451, t63453, t63457, t63459, t63462, t63464);
        let (t64989, t64997) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3455::<F>(t64945, t64959, t64973, t64987, t1024, t1043, t1082, t1083, t1089, t11788, t12149, t16414, t16436, t16458, t1651, t1685, t1689, t19380, t19414, t19479, t19515, t3204, t3278, t3291, t3298, t342, t4743, t4930, t4954, t4984, t5012, t53865, t55649, t55747, t55868, t55991, t64907, t64912, t64916);
        let t65012 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3456::<F>(t42013, t51967, t63274, t63276, t63278, t63281, t63285, t63290, t63293, t63299, t63304, t63308);
        let t65026 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3457::<F>(t41361, t41363, t51973, t51978, t63325, t63328, t63336, t63338, t63340, t63342, t63346, t63351, t63355);
        let t65040 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3458::<F>(t52033, t52035, t52037, t52039, t52041, t52045, t63359, t63361, t63366, t63369, t63371, t63374);
        let t65054 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3459::<F>(t41330, t41332, t52047, t52049, t52051, t63399, t63447, t63451, t63453, t63457, t63459, t63462, t63464);
        let (t65057, t65060, t65071, t65096, t65102) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3460::<F>(t341, t65012, t65026, t65040, t65054, t15648, t1668, t3059, t6258, t3057, t3140, t1035, t1082, t1089, t1093, t11940, t12146, t15670, t15837, t16402, t16436, t16488, t1651, t16584, t19453, t19584, t19856, t20136, t3151, t3204, t3278, t3287, t357, t378, t381, t43350, t4743, t4961, t4977, t4980, t4984, t4995, t4999, t5004, t54695, t55685, t55805, t55948, t64891);
        let (t65122, t65144, t65150) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3461::<F>(t4772, t11249, t6299, t1024, t1082, t11788, t11940, t12073, t15670, t16152, t16183, t16399, t16433, t16449, t16485, t16496, t1651, t16566, t16568, t19556, t19580, t19617, t3059, t3143, t3204, t378, t43154, t43453, t4893, t4954, t4977, t4981, t4982, t55330, t55701, t55764, t6244, t64647, t64772, t65096);
        let (t65186, t65192, t65196) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3462::<F>(t1647, t16565, t3133, t6258, t16183, t1651, t1082, t1089, t12127, t16520, t16537, t16540, t16552, t16569, t16573, t16577, t16581, t19446, t19450, t19501, t19521, t19526, t19593, t20133, t3059, t3204, t3223, t3287, t43438, t43446, t43512, t43568, t4964, t4982, t55646, t55685, t55732, t56049, t6375, t64835);
    (t64989, t64997, t65057, t65060, t65071, t65102, t65122, t65144, t65150, t65186, t65192, t65196)
}
