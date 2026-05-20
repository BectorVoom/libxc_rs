//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta684 (260520-c91 hierarchical CSE).
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
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2251;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2252;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2253;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2254;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2255;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2256;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2257;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2258;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2259;
use chunk9::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2260;
use chunk10::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2261;
use chunk11::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2262;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta684<F: Float>(t29089: F, t5357: F, t21251: F, t7607: F, t21254: F, t104708: F, t104721: F, t104888: F, t104933: F, t20929: F, t21210: F, t29037: F, t5270: F, t5348: F, t5369: F, t5407: F, t97174: F, t97247: F, t20842: F, t7613: F, t1234: F, t30815: F, t20816: F, t7618: F, t29020: F, t5265: F, t104953: F, t104963: F, t104968: F, t1238: F, t20792: F, t21085: F, t21157: F, t26867: F, t7624: F, t97267: F, t97272: F, t20783: F, t26880: F, t5326: F, t8184: F, t20846: F, t26824: F, t29062: F, t5362: F, t1256: F, t30816: F, t104972: F, t112404: F, t20318: F, t26827: F, t29047: F, t29048: F, t29049: F, t29083: F, t5304: F, t6647: F, t97288: F, t97296: F, t30812: F, t104988: F, t104990: F, t20298: F, t20302: F, t21008: F, t21022: F, t21121: F, t21161: F, t21219: F, t21228: F, t29054: F, t6640: F, t97149: F, t97232: F, t104647: F, t104853: F, t104994: F, t104999: F, t105002: F, t105007: F, t105014: F, t20767: F, t20880: F, t21037: F, t21173: F, t21223: F, t29097: F, t29100: F, t5402: F, t112175: F, t112200: F, t112224: F, t112249: F, t112278: F, t112299: F, t112327: F, t112342: F, t112372: F, t112395: F, t112424: F, t104510: F, t105284: F, t112121: F, t1203: F, t1214: F, t1828: F, t1829: F, t21342: F, t21348: F, t2148: F, t2152: F, t225: F, t26889: F, t26949: F, t26969: F, t26994: F, t29109: F, t29119: F, t29136: F, t29141: F, t29149: F, t29159: F, t29199: F, t29201: F, t30751: F, t30767: F, t30771: F, t30849: F, t460: F, t494: F, t5245: F, t5497: F, t6564: F, t7629: F, t7632: F, t7636: F, t7637: F, t7643: F, t7651: F, t7652: F, t8201: F, t8205: F, t97041: F, t105203: F, t105558: F, t105644: F, t1204: F, t20704: F, t2142: F, t21617: F, t26976: F, t26999: F, t29167: F, t29204: F, t29227: F, t29275: F, t29297: F, t30752: F, t30842: F, t30867: F, t30883: F, t30907: F, t5216: F, t5429: F, t6574: F, t6580: F, t7666: F, t8192: F, t8209: F, t96866: F, t105420: F, t111987: F, t111991: F, t1269: F, t21333: F, t2144: F, t27011: F, t27020: F, t27025: F, t29175: F, t29193: F, t29196: F, t29264: F, t29304: F, t30882: F, t30886: F, t30906: F, t5215: F, t5237: F, t5246: F, t6588: F, t6745: F, t8190: F, t96927: F, t96929: F, t96953: F, t96954: F, t96986: F, t97308: F, t112120: F, t3153: F, t1243: F, t30840: F, t1248: F, t1287: F, t1294: F, t20710: F, t20900: F, t26895: F, t26906: F, t26931: F, t26937: F, t29124: F, t29129: F, t29194: F, t29200: F, t29278: F, t30735: F, t30772: F, t30860: F, t30878: F, t3769: F, t3783: F, t5465: F, t5480: F, t6628: F, t7602: F, t7659: F, t7660: F, t96883: F, t97332: F, t1769: F, t105134: F, t105404: F, t105576: F, t1295: F, t1774: F, t1775: F, t20744: F, t21382: F, t21408: F, t2151: F, t29174: F, t29187: F, t30747: F, t30853: F, t30887: F, t5498: F, t97066: F, t97304: F, t1032: F, t6695: F, t1209: F, t105442: F, t20760: F, t21618: F, t21624: F, t26918: F, t27008: F, t29220: F, t29224: F, t30743: F, t30763: F, t30874: F, t5423: F, t7639: F, t7654: F, t8197: F, t8198: F, t97313: F) -> (F, F, F, F, F, F, F, F) {
        let t112447 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2251::<F>(t29089, t5357, t21251, t7607, t21254, t104708, t104721, t104888, t104933, t20929, t21210, t29037, t5270, t5348, t5369, t5407, t97174, t97247);
        let t112467 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2252::<F>(t20842, t7613, t1234, t30815, t20816, t7618, t29020, t5265, t104953, t104963, t104968, t1238, t20792, t21085, t21157, t26867, t7624, t97267, t97272);
        let t112489 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2253::<F>(t20783, t26880, t5326, t8184, t20846, t26824, t29062, t5362, t1256, t30816, t104972, t112404, t1238, t20318, t26827, t29047, t29048, t29049, t29083, t5304, t6647, t97288, t97296);
        let t112515 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2254::<F>(t1256, t30812, t104988, t104990, t20298, t20302, t21008, t21022, t21121, t21161, t21219, t21228, t26867, t29047, t29054, t6640, t97149, t97232);
        let t112531 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2255::<F>(t104647, t104721, t104853, t104888, t104994, t104999, t105002, t105007, t105014, t20767, t20880, t21037, t21173, t21223, t26880, t29097, t29100, t5402);
        let t112535 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2256::<F>(t112175, t112200, t112224, t112249, t112278, t112299, t112327, t112342, t112372, t112395, t112424, t112447, t112467, t112489, t112515, t112531);
        let t112564 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2257::<F>(t104510, t105284, t112121, t112535, t1203, t1214, t1828, t1829, t21342, t21348, t2148, t2152, t225, t26889, t26949, t26969, t26994, t29109, t29119, t29136, t29141, t29149, t29159, t29199, t29201, t30751, t30767, t30771, t30849, t460, t494, t5245, t5497, t6564, t7629, t7632, t7636, t7637, t7643, t7651, t7652, t8201, t8205, t97041);
        let t112602 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2258::<F>(t105203, t105558, t105644, t1203, t1204, t20704, t2142, t21617, t26969, t26976, t26999, t29167, t29204, t29227, t29275, t29297, t30752, t30767, t30842, t30867, t30883, t30907, t5216, t5429, t6574, t6580, t7636, t7651, t7652, t7666, t8192, t8209, t96866);
        let t112645 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2259::<F>(t105420, t111987, t111991, t1214, t1269, t21333, t2144, t2152, t27011, t27020, t27025, t29175, t29193, t29196, t29264, t29275, t29304, t30752, t30849, t30882, t30886, t30906, t5215, t5237, t5246, t6588, t6745, t7636, t7637, t7643, t7652, t8190, t8205, t96927, t96929, t96953, t96954, t96986, t97308);
        let t112697 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2260::<F>(t112120, t3153, t1243, t30840, t1248, t1287, t1294, t1828, t20710, t20900, t26895, t26906, t26931, t26937, t29124, t29129, t29194, t29200, t29278, t30735, t30751, t30772, t30860, t30878, t3769, t3783, t5465, t5480, t5497, t6628, t7602, t7636, t7643, t7651, t7652, t7659, t7660, t8190, t96883, t97332);
        let t112744 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2261::<F>(t2142, t6564, t30840, t460, t1769, t1828, t104510, t105134, t105404, t105576, t1214, t1294, t1295, t1774, t1775, t20744, t21382, t21408, t2151, t26937, t26994, t26999, t29174, t29187, t29227, t29275, t30747, t30853, t30887, t5498, t6588, t7602, t7632, t7636, t7637, t7643, t7652, t97066, t97304);
        let (t112757, t112787) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2262::<F>(t1032, t6695, t2148, t1209, t105442, t111987, t1248, t1287, t20760, t21618, t21624, t26889, t26918, t26994, t27008, t29220, t29224, t29275, t29304, t30743, t30763, t30874, t5245, t5423, t6745, t7602, t7632, t7637, t7639, t7643, t7654, t8190, t8197, t8198, t97313);
    (t112535, t112564, t112602, t112645, t112697, t112744, t112757, t112787)
}
