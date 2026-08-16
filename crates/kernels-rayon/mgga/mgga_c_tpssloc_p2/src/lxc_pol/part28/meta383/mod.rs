//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta383 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1471;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1472;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1473;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1474;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1475;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1476;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1477;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1478;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1479;
use chunk9::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1480;
use chunk10::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1481;
use chunk11::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta383(t1706: f64, t3428: f64, t1184: f64, t460: f64, t4928: f64, t4934: f64, t1714: f64, t3469: f64, t1178: f64, t12606: f64, t1177: f64, t135: f64, t457: f64, t4936: f64, t1174: f64, t3431: f64, t4912: f64, t1090: f64, t7319: f64, t4919: f64, t11531: f64, t11534: f64, t11537: f64, t11541: f64, t11591: f64, t3447: f64, t11583: f64, t3961: f64, t3449: f64, t11529: f64, t1709: f64, t3475: f64, t3432: f64, t4889: f64, t3450: f64, t3966: f64, t14749: f64, t4908: f64, t3448: f64, t3451: f64, t11579: f64, t11584: f64, t3443: f64, t3457: f64, t3461: f64, t14753: f64, t14744: f64, t11588: f64, t14818: f64, t14781: f64, t14710: f64, t11211: f64, t11213: f64, t11215: f64, t11217: f64, t11487: f64, t14713: f64, t14766: f64, t14779: f64, t14784: f64, t14787: f64, t14790: f64, t14793: f64, t14796: f64, t14799: f64, t974: f64, t1716: f64, t698: f64, t3435: f64, t4930: f64, t1420: f64, t1887: f64, t337: f64, t11593: f64, t4904: f64, t11570: f64, t11569: f64, t3452: f64, t3472: f64, t3478: f64, t4899: f64, t11571: f64, t11545: f64, t60: f64, t461: f64, t14726: f64, t11589: f64, t4729: f64, t14736: f64, t4900: f64, t14740: f64, t14731: f64, t11575: f64, t134: f64, t3439: f64, t4724: f64, t11514: f64, t11556: f64, t11558: f64, t11561: f64, t225: f64, t3507: f64, t475: f64, t6739: f64, t1755: f64, t11546: f64, t15026: f64, t3032: f64, t3514: f64, t3572: f64, t5002: f64, t3523: f64, t5005: f64, t5019: f64, t5024: f64, t11147: f64, t11778: f64, t14165: f64, t4582: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15265, t15269, t15274, t15278, t15281) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1471(t1706, t3428, t1184, t460, t4928, t4934, t1714, t3469, t1178, t12606, t1177, t135, t457);
        let t15292 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1472(t15281, t4936, t1174, t3431, t4912, t1090, t7319, t4919, t11531, t11534, t11537, t11541, t11591, t15265, t15269, t15274, t15278, t3447);
        let (t15294, t15300, t15304, t15307) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1473(t11583, t3961, t3449, t11529, t1709, t1174, t1714, t3475, t460, t4934, t3432, t4889);
        let t15330 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1474(t3450, t3966, t3449, t14749, t4908, t3448, t4928, t3451, t11579, t4919, t11584, t1174, t15294, t15300, t15304, t15307, t3443, t3447, t3457, t3461, t4889);
        let (t15332, t15335, t15341, t15357) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1475(t14753, t4908, t14744, t11588, t1714, t3451, t3447, t14818, t14781, t14710, t11211, t11213, t11215, t11217, t11487, t14713, t14766, t14779, t14784, t14787, t14790, t14793, t14796, t14799);
        let (t15360, t15364, t15366, t15374, t15376) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1476(t15357, t457, t460, t974, t1716, t698, t1174, t3435, t4889, t135, t4930, t1420, t1887, t337);
        let t15386 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1477(t11593, t4904, t11570, t3961, t11569, t1174, t15332, t15335, t15341, t15360, t15364, t15366, t15374, t15376, t3447, t3452, t3472, t3478, t4889);
        let (t15391, t15396, t15401, t15403) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1478(t1714, t4899, t11571, t11545, t60, t461, t14726, t11589, t4904, t3447, t11588, t4729);
        let (t15405, t15406, t15409, t15412, t15415, t15420) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1479(t15403, t3447, t14736, t4900, t14740, t14731, t11575, t4904, t134, t3439, t461, t4724);
        let t15423 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1480(t15420, t3447, t11514, t11556, t11558, t11561, t15391, t15396, t15401, t15405, t15406, t15409, t15412, t15415);
        let (t15425, t15426, t15430, t15434, t15437) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1481(t15292, t15330, t15386, t15423, t225, t3507, t475, t6739, t1755, t11546, t14726, t15026, t3032);
        let (t15438, t15446, t15448, t15450, t15452, t15455) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1482(t15437, t3514, t3572, t5002, t3523, t5005, t5019, t5024, t11147, t11778, t14165, t4582);
    (t15425, t15426, t15430, t15434, t15437, t15438, t15446, t15448, t15450, t15452, t15455)
}
