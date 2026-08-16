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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1002(t19049: f64, t3030: f64, t19467: f64, t2989: f64, t981: f64, t19226: f64, t2970: f64, t11404: f64, t11409: f64, t11548: f64, t15252: f64, t15255: f64, t15413: f64, t19227: f64, t19272: f64, t19275: f64, t19276: f64, t19282: f64, t2943: f64, t2944: f64, t2962: f64, t2968: f64, t41667: f64, t41740: f64, t41742: f64, t52443: f64, t6157: f64, t6174: f64, t6177: f64, t63212: f64, t63214: f64, t63216: f64, t63218: f64, t63220: f64, t63222: f64, t63224: f64, t953: f64, t11452: f64, t6173: f64, t2986: f64, t6184: f64, t11399: f64, t11450: f64, t11507: f64, t15263: f64, t15267: f64, t15290: f64, t15339: f64, t15340: f64, t15350: f64, t15400: f64, t1622: f64, t19279: f64, t19283: f64, t2938: f64, t2988: f64, t3006: f64, t3012: f64, t3014: f64, t41662: f64, t41775: f64, t4647: f64, t4670: f64, t4673: f64, t52642: f64, t52830: f64, t6158: f64, t6190: f64, t6209: f64, t63902: f64, t11509: f64, t6205: f64, t19247: f64, t945: f64, t2967: f64, t6152: f64, t11461: f64, t11466: f64, t15234: f64, t1634: f64, t19173: f64, t19303: f64, t19304: f64, t19310: f64, t2963: f64, t2971: f64, t2987: f64, t41751: f64, t41759: f64, t4711: f64, t6206: f64, t63226: f64, t63228: f64, t63579: f64, t63581: f64, t955: f64, t19021: f64, t15104: f64, t15238: f64, t15242: f64, t15274: f64, t15277: f64, t15280: f64, t15284: f64, t15406: f64, t19167: f64, t19263: f64, t19307: f64, t19311: f64, t41756: f64, t4652: f64, t4674: f64, t52809: f64, t52812: f64, t52820: f64, t52825: f64, t63583: f64, t63586: f64, t63589: f64, t63592: f64, t63596: f64, t972: f64, t4669: f64, t19045: f64, t964: f64, t3011: f64, t11456: f64, t15235: f64, t15249: f64, t15259: f64, t15343: f64, t19156: f64, t19266: f64, t3007: f64, t3015: f64, t41785: f64, t4685: f64, t4708: f64, t52264: f64, t52320: f64, t52430: f64, t52511: f64, t52522: f64, t52840: f64, t63612: f64, t954: f64, t973: f64, t974: f64, t2982: f64, t63618: f64, t63620: f64, t63622: f64, t63625: f64, t63628: f64, t63633: f64, t63636: f64, t63638: f64, t63641: f64, t63644: f64, t63647: f64, t63649: f64, t63653: f64, t63660: f64, t63668: f64, t63670: f64, t63673: f64, t63816: f64, t965: f64, t51909: f64, t51911: f64, t51913: f64, t51915: f64, t51917: f64, t51921: f64, t51923: f64, t63238: f64, t63240: f64, t63242: f64, t63246: f64, t63250: f64, t63255: f64, t63260: f64, t41281: f64, t41285: f64, t41287: f64, t41672: f64, t51937: f64, t51942: f64, t63266: f64, t63268: f64, t63274: f64, t63276: f64, t63278: f64, t63281: f64, t63285: f64, t63290: f64, t63293: f64, t41361: f64, t41363: f64, t41690: f64, t51967: f64, t51973: f64, t51978: f64, t63299: f64, t63304: f64, t63308: f64, t63311: f64, t63315: f64, t63320: f64, t63325: f64, t63328: f64, t63332: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63940, t63943, t63975) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3409(t19049, t3030, t19467, t2989, t981, t19226, t2970, t11404, t11409, t11548, t15252, t15255, t15413, t19227, t19272, t19275, t19276, t19282, t2943, t2944, t2962, t2968, t41667, t41740, t41742, t52443, t6157, t6174, t6177, t63212, t63214, t63216, t63218, t63220, t63222, t63224, t953);
        let t64023 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3410(t11452, t6173, t2986, t6184, t11399, t11404, t11450, t11507, t15263, t15267, t15290, t15339, t15340, t15350, t15400, t1622, t19227, t19275, t19279, t19282, t19283, t2938, t2944, t2962, t2968, t2988, t2989, t3006, t3012, t3014, t41662, t41775, t4647, t4670, t4673, t52642, t52830, t6158, t6174, t6190, t6209, t63902);
        let t64068 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3411(t11509, t6205, t19247, t945, t2967, t6152, t11461, t11466, t11507, t15234, t1634, t19173, t19303, t19304, t19310, t2944, t2963, t2968, t2971, t2987, t2988, t3006, t3012, t41751, t41759, t4711, t6174, t6206, t6209, t63226, t63228, t63579, t63581, t955);
        let t64101 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3412(t19021, t3014, t11404, t11461, t15104, t15238, t15242, t15274, t15277, t15280, t15284, t15406, t19167, t19263, t19307, t19311, t2962, t2968, t2987, t3012, t41756, t4652, t4674, t52809, t52812, t52820, t52825, t6158, t63583, t63586, t63589, t63592, t63596, t972);
        let (t64109, t64146) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3413(t4669, t19045, t964, t3011, t6184, t11450, t11456, t11548, t15235, t15249, t15259, t15274, t15339, t15343, t1622, t1634, t19156, t19266, t2943, t2944, t2987, t3007, t3015, t41785, t4685, t4708, t52264, t52320, t52430, t52511, t52522, t52840, t6177, t6190, t6206, t63612, t63902, t953, t954, t973, t974);
        let t64152 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3414(t19167, t2982, t63618, t63620, t63622, t63625, t63628, t63633, t63636, t63638, t63641, t63644, t63647, t63649, t63653, t63660, t63668, t63670, t63673, t63816, t965, t973);
        let t64197 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3415(t51909, t51911, t51913, t51915, t51917, t51921, t51923, t63238, t63240, t63242, t63246, t63250, t63255, t63260);
        let t64212 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3416(t41281, t41285, t41287, t41672, t51937, t51942, t63266, t63268, t63274, t63276, t63278, t63281, t63285, t63290, t63293);
        let t64228 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3417(t41361, t41363, t41690, t51967, t51973, t51978, t63299, t63304, t63308, t63311, t63315, t63320, t63325, t63328, t63332);
    (t63940, t63943, t63975, t64023, t64068, t64101, t64109, t64146, t64152, t64197, t64212, t64228)
}
