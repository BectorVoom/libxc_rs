//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1062 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3792;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3793;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3794;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3795;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3796;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3797;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3798;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3799;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3800;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3801;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3802;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3803;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1062(t12641: f64, t13177: f64, t17974: f64, t17992: f64, t18030: f64, t18102: f64, t18114: f64, t1829: f64, t20700: f64, t20710: f64, t20714: f64, t20744: f64, t21408: f64, t3556: f64, t3729: f64, t3732: f64, t3791: f64, t5237: f64, t5417: f64, t56303: f64, t56327: f64, t56332: f64, t56432: f64, t6564: f64, t6588: f64, t17306: f64, t488: f64, t1269: f64, t1210: f64, t1277: f64, t1295: f64, t17973: f64, t17975: f64, t17995: f64, t18042: f64, t18062: f64, t18103: f64, t20722: f64, t20741: f64, t21394: f64, t3561: f64, t3572: f64, t3585: f64, t5225: f64, t5231: f64, t5245: f64, t5423: f64, t5497: f64, t59464: f64, t21342: f64, t460: f64, t1204: f64, t12633: f64, t1274: f64, t13182: f64, t1775: f64, t18037: f64, t18109: f64, t20704: f64, t20756: f64, t21344: f64, t3552: f64, t3738: f64, t3739: f64, t56396: f64, t56575: f64, t6697: f64, t6744: f64, t1214: f64, t12673: f64, t1294: f64, t17986: f64, t17987: f64, t17998: f64, t18018: f64, t18097: f64, t20697: f64, t21617: f64, t21621: f64, t3576: f64, t3737: f64, t45433: f64, t5246: f64, t56707: f64, t6574: f64, t6703: f64, t6745: f64, t1211: f64, t12628: f64, t18043: f64, t18054: f64, t21348: f64, t21366: f64, t3584: f64, t45438: f64, t5220: f64, t5429: f64, t56393: f64, t60106: f64, t6702: f64, t70120: f64, t71839: f64, t1276: f64, t6587: f64, t487: f64, t70208: f64, t1215: f64, t12666: f64, t17964: f64, t18005: f64, t18047: f64, t21618: f64, t3567: f64, t3575: f64, t5251: f64, t5498: f64, t56570: f64, t6573: f64, t12654: f64, t17331: f64, t18019: f64, t18070: f64, t18087: f64, t18090: f64, t1813: f64, t20748: f64, t21382: f64, t45427: f64, t5216: f64, t5414: f64, t56519: f64, t56607: f64, t60087: f64, t20849: f64, t12603: f64, t12658: f64, t20753: f64, t225: f64, t3736: f64, t45552: f64, t494: f64, t5428: f64, t70202: f64, t72098: f64, t1271: f64, t16750: f64, t1774: f64, t17963: f64, t17999: f64, t18065: f64, t18084: f64, t1828: f64, t20728: f64, t21333: f64, t45449: f64, t495: f64, t56413: f64, t6580: f64, t71179: f64, t1770: f64, t5412: f64, t3555: f64, t6695: f64, t18108: f64, t20760: f64, t21389: f64, t3790: f64, t56588: f64, t70413: f64, t70422: f64, t13181: f64, t16771: f64, t17968: f64, t18073: f64, t21390: f64, t21624: f64, t34934: f64, t45430: f64, t56419: f64, t71606: f64, t12587: f64, t6748: f64, t1300: f64, t198: f64, t336: f64, t3798: f64, t44126: f64, t5023: f64, t6752: f64, t68631: f64, t68633: f64, t68636: f64, t68640: f64, t68673: f64, t68683: f64, t68686: f64, t68689: f64, t68692: f64, t68694: f64, t68696: f64, t68698: f64, t72797: f64, t72832: f64, t72865: f64, t72899: f64) -> f64 {
        let t72925 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3792(t12641, t13177, t17974, t17992, t18030, t18102, t18114, t1829, t20700, t20710, t20714, t20744, t21408, t3556, t3729, t3732, t3791, t5237, t5417, t56303, t56327, t56332, t56432, t6564, t6588);
        let t72956 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3793(t17306, t488, t1269, t6564, t1210, t12641, t1277, t1295, t17973, t17974, t17975, t17992, t17995, t18042, t18062, t18103, t20722, t20741, t21394, t21408, t3561, t3572, t3585, t5225, t5231, t5245, t5423, t5497, t59464);
        let t72986 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3794(t21342, t460, t1204, t12633, t12641, t1274, t1295, t13182, t1775, t18037, t18062, t18109, t1829, t20704, t20714, t20741, t20756, t21344, t3552, t3556, t3738, t3739, t5237, t5417, t5423, t56396, t56575, t6697, t6744);
        let t73020 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3795(t1210, t1214, t12673, t1274, t1277, t1294, t17973, t17974, t17986, t17987, t17998, t18018, t18062, t18097, t20697, t20744, t21617, t21621, t3576, t3585, t3737, t45433, t5246, t56707, t6574, t6703, t6745);
        let t73049 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3796(t1210, t1211, t12628, t12633, t1775, t18030, t18037, t18043, t18054, t20722, t21348, t21366, t21394, t3556, t3561, t3576, t3584, t3737, t45438, t5220, t5246, t5429, t56393, t60106, t6702, t70120, t71839);
        let t73082 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3797(t1276, t6587, t487, t70208, t1210, t1215, t12666, t1277, t1775, t17964, t17973, t18005, t18047, t18109, t18114, t21618, t3561, t3567, t3575, t3584, t3737, t3738, t5225, t5246, t5251, t5498, t56570, t6573, t6588, t6744);
        let t73109 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3798(t12654, t17331, t1775, t17995, t18019, t18070, t18087, t18090, t1813, t20748, t20756, t21382, t3556, t3791, t45427, t5216, t5220, t5231, t5251, t5414, t5429, t56519, t56607, t60087, t6745);
        let t73146 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3799(t1269, t20849, t1210, t1211, t1215, t12603, t12658, t1274, t17964, t17986, t20697, t20753, t21621, t225, t3576, t3585, t3736, t3737, t3738, t3791, t45552, t460, t494, t5245, t5417, t5428, t6587, t6588, t6702, t6703, t70202, t72098);
        let t73177 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3800(t1210, t1271, t1277, t13177, t16750, t1774, t1775, t17963, t17999, t18037, t18065, t18084, t1828, t20728, t20748, t21333, t3556, t3572, t45449, t495, t5220, t5237, t5251, t5429, t56413, t6580, t71179);
        let t73210 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3801(t1770, t5412, t3555, t6695, t1211, t1215, t12654, t1277, t1295, t17986, t18090, t18097, t18108, t20700, t20760, t21389, t3561, t3567, t3739, t3790, t5220, t5231, t5423, t56588, t6573, t6703, t70413, t70422);
        let t73244 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3802(t1204, t6695, t1276, t6573, t1211, t12628, t1295, t13181, t16771, t17968, t17986, t17995, t18019, t18073, t20710, t20760, t21390, t21624, t34934, t3572, t3575, t3732, t45430, t5225, t5251, t56327, t56419, t6574, t6702, t71606);
        let t73260 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3803(t12587, t6748, t1300, t198, t336, t3798, t44126, t5023, t6752, t68631, t68633, t68636, t68640, t68673, t68683, t68686, t68689, t68692, t68694, t68696, t68698, t72797, t72832, t72865, t72899, t72925, t72956, t72986, t73020, t73049, t73082, t73109, t73146, t73177, t73210, t73244);
    t73260
}
