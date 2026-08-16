//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta682 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2233;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2234;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2235;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2236;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2237;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2238;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2239;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta682<F: Float>(t2142: F, t6628: F, t3153: F, t11249: F, t5219: F, t7635: F, t6622: F, t73: F, t104482: F, t105383: F, t1203: F, t1248: F, t1287: F, t21472: F, t21582: F, t21586: F, t21595: F, t26889: F, t26895: F, t29159: F, t29175: F, t29194: F, t29195: F, t30739: F, t30751: F, t30840: F, t30853: F, t5458: F, t6744: F, t7627: F, t7636: F, t7637: F, t7651: F, t7652: F, t96929: F, t96953: F, t97041: F, t97308: F, t97313: F, t97314: F, t97318: F, t97319: F, t97397: F, t97398: F, t1209: F, t104510: F, t105519: F, t1215: F, t1294: F, t1794: F, t1829: F, t20722: F, t21366: F, t26922: F, t26949: F, t26976: F, t27020: F, t29135: F, t29174: F, t29178: F, t29200: F, t29204: F, t30743: F, t30744: F, t30763: F, t30850: F, t3555: F, t5284: F, t5465: F, t5480: F, t6703: F, t7602: F, t8197: F, t8198: F, t96927: F, t97082: F, t104465: F, t105365: F, t105530: F, t1214: F, t1774: F, t21483: F, t21557: F, t26994: F, t27011: F, t29109: F, t29160: F, t29207: F, t29213: F, t30735: F, t30740: F, t30866: F, t5498: F, t6563: F, t6580: F, t7643: F, t96954: F, t96979: F, t96982: F, t96986: F, t97050: F, t97304: F, t3303: F, t5341: F, t5333: F, t104505: F, t1769: F, t20728: F, t26969: F, t26979: F, t29136: F, t29141: F, t29163: F, t29166: F, t29237: F, t29278: F, t29283: F, t30747: F, t30748: F, t30849: F, t5497: F, t6574: F, t8208: F, t97019: F, t20849: F, t1276: F, t2148: F, t3140: F, t6695: F, t105509: F, t105512: F, t105558: F, t27008: F, t29217: F, t29220: F, t29268: F, t29275: F, t29279: F, t29287: F, t29293: F, t29301: F, t29308: F, t30757: F, t5231: F, t5237: F, t7662: F, t97358: F, t97475: F, t1770: F, t8190: F, t104549: F, t105220: F, t1295: F, t20714: F, t21082: F, t21390: F, t26937: F, t27025: F, t29129: F, t29183: F, t29233: F, t29247: F, t30764: F, t30767: F, t30768: F, t30870: F, t30907: F, t7666: F, t8213: F, t97348: F, t97377: F, t97422: F, t30881: F, t3565: F, t104524: F, t105350: F, t105579: F, t20741: F, t21332: F, t26918: F, t29251: F, t29272: F, t30736: F, t30893: F, t6702: F, t7645: F, t8202: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t111815, t111825, t111844, t111845, t111864) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2233::<F>(t2142, t6628, t3153, t11249, t5219, t7635, t6622, t73, t104482, t105383, t1203, t1248, t1287, t21472, t21582, t21586, t21595, t26889, t26895, t29159, t29175, t29194, t29195, t30739, t30751, t30840, t30853, t5458, t6744, t7627, t7636, t7637, t7651, t7652, t96929, t96953, t97041, t97308, t97313, t97314, t97318, t97319, t97397, t97398);
        let t111913 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2234::<F>(t1209, t30840, t111844, t3153, t104510, t105519, t1215, t1287, t1294, t1794, t1829, t20722, t21366, t26889, t26922, t26949, t26976, t27020, t29135, t29174, t29178, t29194, t29200, t29204, t30739, t30743, t30744, t30763, t30850, t3555, t5284, t5465, t5480, t6703, t7602, t7636, t7652, t8197, t8198, t96927, t97082);
        let t111959 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2235::<F>(t104465, t105365, t105530, t111815, t1203, t1214, t1774, t1829, t21483, t21557, t26949, t26994, t27011, t29109, t29160, t29195, t29200, t29207, t29213, t30735, t30740, t30743, t30866, t5498, t6563, t6580, t7627, t7636, t7637, t7643, t96954, t96979, t96982, t96986, t97050, t97304);
        let (t111987, t111991, t112009) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2236::<F>(t3303, t5341, t5333, t104505, t105365, t111845, t1203, t1287, t1769, t1794, t20728, t26895, t26922, t26969, t26979, t26994, t29109, t29136, t29141, t29163, t29166, t29237, t29278, t29283, t30747, t30748, t30763, t30849, t30853, t5284, t5497, t6574, t7602, t7636, t7637, t7651, t8208, t96953, t96979, t97019, t97304, t97318);
        let t112051 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2237::<F>(t20849, t2142, t1276, t2148, t3140, t6695, t105509, t105512, t105530, t105558, t1203, t1214, t1215, t27008, t29136, t29217, t29220, t29268, t29275, t29279, t29287, t29293, t29301, t29308, t30739, t30757, t5231, t5237, t6703, t7637, t7662, t96927, t96954, t97358, t97475);
        let t112092 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2238::<F>(t1770, t8190, t104549, t105220, t1294, t1295, t20714, t21082, t21390, t2142, t26937, t26976, t27025, t29129, t29136, t29183, t29233, t29247, t30735, t30744, t30763, t30764, t30767, t30768, t30870, t30907, t7637, t7643, t7651, t7652, t7666, t8213, t96929, t97348, t97377, t97422);
        let (t112120, t112121, t112138) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2239::<F>(t1794, t8190, t73, t30881, t3565, t7635, t104524, t105350, t105579, t1214, t1294, t1829, t20741, t21332, t2142, t26895, t26918, t26922, t26949, t26969, t26979, t29129, t29141, t29166, t29251, t29272, t30736, t30747, t30840, t30893, t5231, t5458, t6702, t7602, t7627, t7636, t7637, t7645, t7651, t7652, t8202);
    (t111825, t111864, t111913, t111959, t111987, t111991, t112009, t112051, t112092, t112120, t112121, t112138)
}
