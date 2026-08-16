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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta684(t29089: f64, t5357: f64, t21251: f64, t7607: f64, t21254: f64, t104708: f64, t104721: f64, t104888: f64, t104933: f64, t20929: f64, t21210: f64, t29037: f64, t5270: f64, t5348: f64, t5369: f64, t5407: f64, t97174: f64, t97247: f64, t20842: f64, t7613: f64, t1234: f64, t30815: f64, t20816: f64, t7618: f64, t29020: f64, t5265: f64, t104953: f64, t104963: f64, t104968: f64, t1238: f64, t20792: f64, t21085: f64, t21157: f64, t26867: f64, t7624: f64, t97267: f64, t97272: f64, t20783: f64, t26880: f64, t5326: f64, t8184: f64, t20846: f64, t26824: f64, t29062: f64, t5362: f64, t1256: f64, t30816: f64, t104972: f64, t112404: f64, t20318: f64, t26827: f64, t29047: f64, t29048: f64, t29049: f64, t29083: f64, t5304: f64, t6647: f64, t97288: f64, t97296: f64, t30812: f64, t104988: f64, t104990: f64, t20298: f64, t20302: f64, t21008: f64, t21022: f64, t21121: f64, t21161: f64, t21219: f64, t21228: f64, t29054: f64, t6640: f64, t97149: f64, t97232: f64, t104647: f64, t104853: f64, t104994: f64, t104999: f64, t105002: f64, t105007: f64, t105014: f64, t20767: f64, t20880: f64, t21037: f64, t21173: f64, t21223: f64, t29097: f64, t29100: f64, t5402: f64, t112175: f64, t112200: f64, t112224: f64, t112249: f64, t112278: f64, t112299: f64, t112327: f64, t112342: f64, t112372: f64, t112395: f64, t112424: f64, t104510: f64, t105284: f64, t112121: f64, t1203: f64, t1214: f64, t1828: f64, t1829: f64, t21342: f64, t21348: f64, t2148: f64, t2152: f64, t225: f64, t26889: f64, t26949: f64, t26969: f64, t26994: f64, t29109: f64, t29119: f64, t29136: f64, t29141: f64, t29149: f64, t29159: f64, t29199: f64, t29201: f64, t30751: f64, t30767: f64, t30771: f64, t30849: f64, t460: f64, t494: f64, t5245: f64, t5497: f64, t6564: f64, t7629: f64, t7632: f64, t7636: f64, t7637: f64, t7643: f64, t7651: f64, t7652: f64, t8201: f64, t8205: f64, t97041: f64, t105203: f64, t105558: f64, t105644: f64, t1204: f64, t20704: f64, t2142: f64, t21617: f64, t26976: f64, t26999: f64, t29167: f64, t29204: f64, t29227: f64, t29275: f64, t29297: f64, t30752: f64, t30842: f64, t30867: f64, t30883: f64, t30907: f64, t5216: f64, t5429: f64, t6574: f64, t6580: f64, t7666: f64, t8192: f64, t8209: f64, t96866: f64, t105420: f64, t111987: f64, t111991: f64, t1269: f64, t21333: f64, t2144: f64, t27011: f64, t27020: f64, t27025: f64, t29175: f64, t29193: f64, t29196: f64, t29264: f64, t29304: f64, t30882: f64, t30886: f64, t30906: f64, t5215: f64, t5237: f64, t5246: f64, t6588: f64, t6745: f64, t8190: f64, t96927: f64, t96929: f64, t96953: f64, t96954: f64, t96986: f64, t97308: f64, t112120: f64, t3153: f64, t1243: f64, t30840: f64, t1248: f64, t1287: f64, t1294: f64, t20710: f64, t20900: f64, t26895: f64, t26906: f64, t26931: f64, t26937: f64, t29124: f64, t29129: f64, t29194: f64, t29200: f64, t29278: f64, t30735: f64, t30772: f64, t30860: f64, t30878: f64, t3769: f64, t3783: f64, t5465: f64, t5480: f64, t6628: f64, t7602: f64, t7659: f64, t7660: f64, t96883: f64, t97332: f64, t1769: f64, t105134: f64, t105404: f64, t105576: f64, t1295: f64, t1774: f64, t1775: f64, t20744: f64, t21382: f64, t21408: f64, t2151: f64, t29174: f64, t29187: f64, t30747: f64, t30853: f64, t30887: f64, t5498: f64, t97066: f64, t97304: f64, t1032: f64, t6695: f64, t1209: f64, t105442: f64, t20760: f64, t21618: f64, t21624: f64, t26918: f64, t27008: f64, t29220: f64, t29224: f64, t30743: f64, t30763: f64, t30874: f64, t5423: f64, t7639: f64, t7654: f64, t8197: f64, t8198: f64, t97313: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t112447 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2251(t29089, t5357, t21251, t7607, t21254, t104708, t104721, t104888, t104933, t20929, t21210, t29037, t5270, t5348, t5369, t5407, t97174, t97247);
        let t112467 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2252(t20842, t7613, t1234, t30815, t20816, t7618, t29020, t5265, t104953, t104963, t104968, t1238, t20792, t21085, t21157, t26867, t7624, t97267, t97272);
        let t112489 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2253(t20783, t26880, t5326, t8184, t20846, t26824, t29062, t5362, t1256, t30816, t104972, t112404, t1238, t20318, t26827, t29047, t29048, t29049, t29083, t5304, t6647, t97288, t97296);
        let t112515 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2254(t1256, t30812, t104988, t104990, t20298, t20302, t21008, t21022, t21121, t21161, t21219, t21228, t26867, t29047, t29054, t6640, t97149, t97232);
        let t112531 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2255(t104647, t104721, t104853, t104888, t104994, t104999, t105002, t105007, t105014, t20767, t20880, t21037, t21173, t21223, t26880, t29097, t29100, t5402);
        let t112535 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2256(t112175, t112200, t112224, t112249, t112278, t112299, t112327, t112342, t112372, t112395, t112424, t112447, t112467, t112489, t112515, t112531);
        let t112564 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2257(t104510, t105284, t112121, t112535, t1203, t1214, t1828, t1829, t21342, t21348, t2148, t2152, t225, t26889, t26949, t26969, t26994, t29109, t29119, t29136, t29141, t29149, t29159, t29199, t29201, t30751, t30767, t30771, t30849, t460, t494, t5245, t5497, t6564, t7629, t7632, t7636, t7637, t7643, t7651, t7652, t8201, t8205, t97041);
        let t112602 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2258(t105203, t105558, t105644, t1203, t1204, t20704, t2142, t21617, t26969, t26976, t26999, t29167, t29204, t29227, t29275, t29297, t30752, t30767, t30842, t30867, t30883, t30907, t5216, t5429, t6574, t6580, t7636, t7651, t7652, t7666, t8192, t8209, t96866);
        let t112645 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2259(t105420, t111987, t111991, t1214, t1269, t21333, t2144, t2152, t27011, t27020, t27025, t29175, t29193, t29196, t29264, t29275, t29304, t30752, t30849, t30882, t30886, t30906, t5215, t5237, t5246, t6588, t6745, t7636, t7637, t7643, t7652, t8190, t8205, t96927, t96929, t96953, t96954, t96986, t97308);
        let t112697 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2260(t112120, t3153, t1243, t30840, t1248, t1287, t1294, t1828, t20710, t20900, t26895, t26906, t26931, t26937, t29124, t29129, t29194, t29200, t29278, t30735, t30751, t30772, t30860, t30878, t3769, t3783, t5465, t5480, t5497, t6628, t7602, t7636, t7643, t7651, t7652, t7659, t7660, t8190, t96883, t97332);
        let t112744 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2261(t2142, t6564, t30840, t460, t1769, t1828, t104510, t105134, t105404, t105576, t1214, t1294, t1295, t1774, t1775, t20744, t21382, t21408, t2151, t26937, t26994, t26999, t29174, t29187, t29227, t29275, t30747, t30853, t30887, t5498, t6588, t7602, t7632, t7636, t7637, t7643, t7652, t97066, t97304);
        let (t112757, t112787) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2262(t1032, t6695, t2148, t1209, t105442, t111987, t1248, t1287, t20760, t21618, t21624, t26889, t26918, t26994, t27008, t29220, t29224, t29275, t29304, t30743, t30763, t30874, t5245, t5423, t6745, t7602, t7632, t7637, t7639, t7643, t7654, t8190, t8197, t8198, t97313);
    (t112535, t112564, t112602, t112645, t112697, t112744, t112757, t112787)
}
