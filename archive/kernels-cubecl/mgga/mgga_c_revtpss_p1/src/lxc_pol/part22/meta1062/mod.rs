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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1062<F: Float>(t12641: F, t13177: F, t17974: F, t17992: F, t18030: F, t18102: F, t18114: F, t1829: F, t20700: F, t20710: F, t20714: F, t20744: F, t21408: F, t3556: F, t3729: F, t3732: F, t3791: F, t5237: F, t5417: F, t56303: F, t56327: F, t56332: F, t56432: F, t6564: F, t6588: F, t17306: F, t488: F, t1269: F, t1210: F, t1277: F, t1295: F, t17973: F, t17975: F, t17995: F, t18042: F, t18062: F, t18103: F, t20722: F, t20741: F, t21394: F, t3561: F, t3572: F, t3585: F, t5225: F, t5231: F, t5245: F, t5423: F, t5497: F, t59464: F, t21342: F, t460: F, t1204: F, t12633: F, t1274: F, t13182: F, t1775: F, t18037: F, t18109: F, t20704: F, t20756: F, t21344: F, t3552: F, t3738: F, t3739: F, t56396: F, t56575: F, t6697: F, t6744: F, t1214: F, t12673: F, t1294: F, t17986: F, t17987: F, t17998: F, t18018: F, t18097: F, t20697: F, t21617: F, t21621: F, t3576: F, t3737: F, t45433: F, t5246: F, t56707: F, t6574: F, t6703: F, t6745: F, t1211: F, t12628: F, t18043: F, t18054: F, t21348: F, t21366: F, t3584: F, t45438: F, t5220: F, t5429: F, t56393: F, t60106: F, t6702: F, t70120: F, t71839: F, t1276: F, t6587: F, t487: F, t70208: F, t1215: F, t12666: F, t17964: F, t18005: F, t18047: F, t21618: F, t3567: F, t3575: F, t5251: F, t5498: F, t56570: F, t6573: F, t12654: F, t17331: F, t18019: F, t18070: F, t18087: F, t18090: F, t1813: F, t20748: F, t21382: F, t45427: F, t5216: F, t5414: F, t56519: F, t56607: F, t60087: F, t20849: F, t12603: F, t12658: F, t20753: F, t225: F, t3736: F, t45552: F, t494: F, t5428: F, t70202: F, t72098: F, t1271: F, t16750: F, t1774: F, t17963: F, t17999: F, t18065: F, t18084: F, t1828: F, t20728: F, t21333: F, t45449: F, t495: F, t56413: F, t6580: F, t71179: F, t1770: F, t5412: F, t3555: F, t6695: F, t18108: F, t20760: F, t21389: F, t3790: F, t56588: F, t70413: F, t70422: F, t13181: F, t16771: F, t17968: F, t18073: F, t21390: F, t21624: F, t34934: F, t45430: F, t56419: F, t71606: F, t12587: F, t6748: F, t1300: F, t198: F, t336: F, t3798: F, t44126: F, t5023: F, t6752: F, t68631: F, t68633: F, t68636: F, t68640: F, t68673: F, t68683: F, t68686: F, t68689: F, t68692: F, t68694: F, t68696: F, t68698: F, t72797: F, t72832: F, t72865: F, t72899: F) -> F {
        let t72925 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3792::<F>(t12641, t13177, t17974, t17992, t18030, t18102, t18114, t1829, t20700, t20710, t20714, t20744, t21408, t3556, t3729, t3732, t3791, t5237, t5417, t56303, t56327, t56332, t56432, t6564, t6588);
        let t72956 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3793::<F>(t17306, t488, t1269, t6564, t1210, t12641, t1277, t1295, t17973, t17974, t17975, t17992, t17995, t18042, t18062, t18103, t20722, t20741, t21394, t21408, t3561, t3572, t3585, t5225, t5231, t5245, t5423, t5497, t59464);
        let t72986 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3794::<F>(t21342, t460, t1204, t12633, t12641, t1274, t1295, t13182, t1775, t18037, t18062, t18109, t1829, t20704, t20714, t20741, t20756, t21344, t3552, t3556, t3738, t3739, t5237, t5417, t5423, t56396, t56575, t6697, t6744);
        let t73020 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3795::<F>(t1210, t1214, t12673, t1274, t1277, t1294, t17973, t17974, t17986, t17987, t17998, t18018, t18062, t18097, t20697, t20744, t21617, t21621, t3576, t3585, t3737, t45433, t5246, t56707, t6574, t6703, t6745);
        let t73049 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3796::<F>(t1210, t1211, t12628, t12633, t1775, t18030, t18037, t18043, t18054, t20722, t21348, t21366, t21394, t3556, t3561, t3576, t3584, t3737, t45438, t5220, t5246, t5429, t56393, t60106, t6702, t70120, t71839);
        let t73082 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3797::<F>(t1276, t6587, t487, t70208, t1210, t1215, t12666, t1277, t1775, t17964, t17973, t18005, t18047, t18109, t18114, t21618, t3561, t3567, t3575, t3584, t3737, t3738, t5225, t5246, t5251, t5498, t56570, t6573, t6588, t6744);
        let t73109 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3798::<F>(t12654, t17331, t1775, t17995, t18019, t18070, t18087, t18090, t1813, t20748, t20756, t21382, t3556, t3791, t45427, t5216, t5220, t5231, t5251, t5414, t5429, t56519, t56607, t60087, t6745);
        let t73146 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3799::<F>(t1269, t20849, t1210, t1211, t1215, t12603, t12658, t1274, t17964, t17986, t20697, t20753, t21621, t225, t3576, t3585, t3736, t3737, t3738, t3791, t45552, t460, t494, t5245, t5417, t5428, t6587, t6588, t6702, t6703, t70202, t72098);
        let t73177 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3800::<F>(t1210, t1271, t1277, t13177, t16750, t1774, t1775, t17963, t17999, t18037, t18065, t18084, t1828, t20728, t20748, t21333, t3556, t3572, t45449, t495, t5220, t5237, t5251, t5429, t56413, t6580, t71179);
        let t73210 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3801::<F>(t1770, t5412, t3555, t6695, t1211, t1215, t12654, t1277, t1295, t17986, t18090, t18097, t18108, t20700, t20760, t21389, t3561, t3567, t3739, t3790, t5220, t5231, t5423, t56588, t6573, t6703, t70413, t70422);
        let t73244 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3802::<F>(t1204, t6695, t1276, t6573, t1211, t12628, t1295, t13181, t16771, t17968, t17986, t17995, t18019, t18073, t20710, t20760, t21390, t21624, t34934, t3572, t3575, t3732, t45430, t5225, t5251, t56327, t56419, t6574, t6702, t71606);
        let t73260 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3803::<F>(t12587, t6748, t1300, t198, t336, t3798, t44126, t5023, t6752, t68631, t68633, t68636, t68640, t68673, t68683, t68686, t68689, t68692, t68694, t68696, t68698, t72797, t72832, t72865, t72899, t72925, t72956, t72986, t73020, t73049, t73082, t73109, t73146, t73177, t73210, t73244);
    t73260
}
