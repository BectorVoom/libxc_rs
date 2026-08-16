//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta777 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2655;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2656;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2657;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2658;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2659;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2660;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2661;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2662;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta777(t12283: f64, t20460: f64, t20565: f64, t3866: f64, t1827: f64, t57056: f64, t20492: f64, t39944: f64, t12215: f64, t1307: f64, t16394: f64, t1810: f64, t19631: f64, t19962: f64, t19996: f64, t20511: f64, t210: f64, t3733: f64, t40025: f64, t5187: f64, t5226: f64, t5240: f64, t5259: f64, t5293: f64, t53882: f64, t53901: f64, t56878: f64, t6347: f64, t6370: f64, t16288: f64, t6417: f64, t12385: f64, t20497: f64, t120: f64, t12369: f64, t12429: f64, t1352: f64, t1363: f64, t16278: f64, t19735: f64, t19871: f64, t19951: f64, t19989: f64, t20356: f64, t20416: f64, t20454: f64, t3803: f64, t3805: f64, t40070: f64, t5246: f64, t5248: f64, t53918: f64, t53920: f64, t54023: f64, t54162: f64, t6390: f64, t6396: f64, t6422: f64, t74120: f64, t820: f64, t20433: f64, t16336: f64, t6431: f64, t1831: f64, t57021: f64, t53945: f64, t20450: f64, t16233: f64, t19873: f64, t19876: f64, t20000: f64, t40192: f64, t5250: f64, t5303: f64, t53928: f64, t56685: f64, t56687: f64, t57081: f64, t57568: f64, t74090: f64, t20595: f64, t68: f64, t1340: f64, t20556: f64, t3799: f64, t20570: f64, t1362: f64, t1354: f64, t1369: f64, t16321: f64, t19868: f64, t19904: f64, t19930: f64, t19991: f64, t20479: f64, t3783: f64, t39936: f64, t40035: f64, t5235: f64, t5314: f64, t57024: f64, t25: f64, t1298: f64, t15989: f64, t15992: f64, t16557: f64, t19606: f64, t20216: f64, t20376: f64, t2219: f64, t3704: f64, t39861: f64, t5170: f64, t606: f64, t67059: f64, t73975: f64, t73978: f64, zeta_threshold: f64, t28: f64, t1081: f64, t1302: f64, t16003: f64, t16006: f64, t18196: f64, t19618: f64, t20385: f64, t20390: f64, t3711: f64, t39877: f64, t5178: f64, t71090: f64, t73995: f64, t73998: f64, t20512: f64, t40021: f64, t1351: f64, t1367: f64, t16225: f64, t16305: f64, t16311: f64, t19855: f64, t20473: f64, t5289: f64, t5310: f64, t53985: f64, t53998: f64, t56693: f64, t56710: f64, t56738: f64, t56924: f64, t57342: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t74216 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2655(t12283, t20460, t20565, t3866, t1827, t57056, t20492, t39944, t12215, t1307, t16394, t1810, t19631, t19962, t19996, t20511, t210, t3733, t40025, t5187, t5226, t5240, t5259, t5293, t53882, t53901, t56878, t6347, t6370);
        let t74253 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2656(t16288, t6417, t12385, t20497, t120, t12369, t12429, t1307, t1352, t1363, t16278, t16394, t19735, t19871, t19951, t19989, t20356, t20416, t20454, t3803, t3805, t40070, t5246, t5248, t53918, t53920, t54023, t54162, t6390, t6396, t6422, t74120, t820);
        let t74286 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2657(t20433, t3866, t16336, t6431, t1831, t57021, t53945, t6396, t12283, t20450, t16233, t19871, t19873, t19876, t20000, t3805, t40192, t5246, t5248, t5250, t5303, t53928, t56685, t56687, t56878, t57081, t57568, t74090, t74120);
        let (t74289, t74316) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2658(t20595, t68, t1340, t20556, t3799, t20570, t1362, t1354, t1369, t16278, t16321, t16394, t1831, t19868, t19904, t19930, t19991, t20479, t20492, t3783, t39936, t40035, t5235, t5240, t5314, t57024, t6417, t6431);
        let t74335 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2659(t25, t1298, t15989, t15992, t16557, t19606, t20216, t20376, t2219, t3704, t39861, t5170, t606, t67059, t73975, t73978, zeta_threshold);
        let t74353 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2660(t28, t1081, t1302, t16003, t16006, t18196, t19618, t20385, t20390, t2219, t3711, t39877, t5178, t71090, t73995, t73998, zeta_threshold);
        let t74355 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2661(t74335, t74353);
        let t74386 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2662(t20512, t40021, t1351, t6347, t16288, t6422, t1363, t1367, t16225, t16233, t16305, t16311, t1827, t19855, t19904, t20473, t5246, t5289, t5310, t53985, t53998, t56693, t56710, t56738, t56924, t57342, t74355, t820);
    (t74216, t74253, t74286, t74289, t74316, t74355, t74386)
}
