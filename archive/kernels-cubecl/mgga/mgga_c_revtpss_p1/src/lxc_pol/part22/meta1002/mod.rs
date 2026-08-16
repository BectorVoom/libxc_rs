//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1002 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3409;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3410;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3411;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3412;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3413;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3414;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3415;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3416;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3417;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1002<F: Float>(t19049: F, t3030: F, t19467: F, t2989: F, t981: F, t19226: F, t2970: F, t11404: F, t11409: F, t11548: F, t15252: F, t15255: F, t15413: F, t19227: F, t19272: F, t19275: F, t19276: F, t19282: F, t2943: F, t2944: F, t2962: F, t2968: F, t41667: F, t41740: F, t41742: F, t52443: F, t6157: F, t6174: F, t6177: F, t63212: F, t63214: F, t63216: F, t63218: F, t63220: F, t63222: F, t63224: F, t953: F, t11452: F, t6173: F, t2986: F, t6184: F, t11399: F, t11450: F, t11507: F, t15263: F, t15267: F, t15290: F, t15339: F, t15340: F, t15350: F, t15400: F, t1622: F, t19279: F, t19283: F, t2938: F, t2988: F, t3006: F, t3012: F, t3014: F, t41662: F, t41775: F, t4647: F, t4670: F, t4673: F, t52642: F, t52830: F, t6158: F, t6190: F, t6209: F, t63902: F, t11509: F, t6205: F, t19247: F, t945: F, t2967: F, t6152: F, t11461: F, t11466: F, t15234: F, t1634: F, t19173: F, t19303: F, t19304: F, t19310: F, t2963: F, t2971: F, t2987: F, t41751: F, t41759: F, t4711: F, t6206: F, t63226: F, t63228: F, t63579: F, t63581: F, t955: F, t19021: F, t15104: F, t15238: F, t15242: F, t15274: F, t15277: F, t15280: F, t15284: F, t15406: F, t19167: F, t19263: F, t19307: F, t19311: F, t41756: F, t4652: F, t4674: F, t52809: F, t52812: F, t52820: F, t52825: F, t63583: F, t63586: F, t63589: F, t63592: F, t63596: F, t972: F, t4669: F, t19045: F, t964: F, t3011: F, t11456: F, t15235: F, t15249: F, t15259: F, t15343: F, t19156: F, t19266: F, t3007: F, t3015: F, t41785: F, t4685: F, t4708: F, t52264: F, t52320: F, t52430: F, t52511: F, t52522: F, t52840: F, t63612: F, t954: F, t973: F, t974: F, t2982: F, t63618: F, t63620: F, t63622: F, t63625: F, t63628: F, t63633: F, t63636: F, t63638: F, t63641: F, t63644: F, t63647: F, t63649: F, t63653: F, t63660: F, t63668: F, t63670: F, t63673: F, t63816: F, t965: F, t51909: F, t51911: F, t51913: F, t51915: F, t51917: F, t51921: F, t51923: F, t63238: F, t63240: F, t63242: F, t63246: F, t63250: F, t63255: F, t63260: F, t41281: F, t41285: F, t41287: F, t41672: F, t51937: F, t51942: F, t63266: F, t63268: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F, t41361: F, t41363: F, t41690: F, t51967: F, t51973: F, t51978: F, t63299: F, t63304: F, t63308: F, t63311: F, t63315: F, t63320: F, t63325: F, t63328: F, t63332: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t63940, t63943, t63975) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3409::<F>(t19049, t3030, t19467, t2989, t981, t19226, t2970, t11404, t11409, t11548, t15252, t15255, t15413, t19227, t19272, t19275, t19276, t19282, t2943, t2944, t2962, t2968, t41667, t41740, t41742, t52443, t6157, t6174, t6177, t63212, t63214, t63216, t63218, t63220, t63222, t63224, t953);
        let t64023 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3410::<F>(t11452, t6173, t2986, t6184, t11399, t11404, t11450, t11507, t15263, t15267, t15290, t15339, t15340, t15350, t15400, t1622, t19227, t19275, t19279, t19282, t19283, t2938, t2944, t2962, t2968, t2988, t2989, t3006, t3012, t3014, t41662, t41775, t4647, t4670, t4673, t52642, t52830, t6158, t6174, t6190, t6209, t63902);
        let t64068 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3411::<F>(t11509, t6205, t19247, t945, t2967, t6152, t11461, t11466, t11507, t15234, t1634, t19173, t19303, t19304, t19310, t2944, t2963, t2968, t2971, t2987, t2988, t3006, t3012, t41751, t41759, t4711, t6174, t6206, t6209, t63226, t63228, t63579, t63581, t955);
        let t64101 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3412::<F>(t19021, t3014, t11404, t11461, t15104, t15238, t15242, t15274, t15277, t15280, t15284, t15406, t19167, t19263, t19307, t19311, t2962, t2968, t2987, t3012, t41756, t4652, t4674, t52809, t52812, t52820, t52825, t6158, t63583, t63586, t63589, t63592, t63596, t972);
        let (t64109, t64146) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3413::<F>(t4669, t19045, t964, t3011, t6184, t11450, t11456, t11548, t15235, t15249, t15259, t15274, t15339, t15343, t1622, t1634, t19156, t19266, t2943, t2944, t2987, t3007, t3015, t41785, t4685, t4708, t52264, t52320, t52430, t52511, t52522, t52840, t6177, t6190, t6206, t63612, t63902, t953, t954, t973, t974);
        let t64152 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3414::<F>(t19167, t2982, t63618, t63620, t63622, t63625, t63628, t63633, t63636, t63638, t63641, t63644, t63647, t63649, t63653, t63660, t63668, t63670, t63673, t63816, t965, t973);
        let t64197 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3415::<F>(t51909, t51911, t51913, t51915, t51917, t51921, t51923, t63238, t63240, t63242, t63246, t63250, t63255, t63260);
        let t64212 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3416::<F>(t41281, t41285, t41287, t41672, t51937, t51942, t63266, t63268, t63274, t63276, t63278, t63281, t63285, t63290, t63293);
        let t64228 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3417::<F>(t41361, t41363, t41690, t51967, t51973, t51978, t63299, t63304, t63308, t63311, t63315, t63320, t63325, t63328, t63332);
    (t63940, t63943, t63975, t64023, t64068, t64101, t64109, t64146, t64152, t64197, t64212, t64228)
}
