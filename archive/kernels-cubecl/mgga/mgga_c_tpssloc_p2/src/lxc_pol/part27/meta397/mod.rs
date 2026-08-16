//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta397 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1630;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1631;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1632;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1633;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1634;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1635;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1636;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1637;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1638;
use chunk9::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1639;
use chunk10::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1640;
use chunk11::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1641;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta397<F: Float>(t1706: F, t3428: F, t1184: F, t460: F, t4928: F, t4934: F, t1714: F, t3469: F, t1178: F, t12606: F, t1177: F, t135: F, t457: F, t4936: F, t1174: F, t3431: F, t4912: F, t1090: F, t7319: F, t4919: F, t11531: F, t11534: F, t11537: F, t11541: F, t11591: F, t3447: F, t11583: F, t3961: F, t3449: F, t11529: F, t1709: F, t3475: F, t3432: F, t4889: F, t3450: F, t3966: F, t14749: F, t4908: F, t3448: F, t3451: F, t11579: F, t11584: F, t3443: F, t3457: F, t3461: F, t14753: F, t14744: F, t11588: F, t14818: F, t14781: F, t14710: F, t11211: F, t11213: F, t11215: F, t11217: F, t11487: F, t14713: F, t14766: F, t14779: F, t14784: F, t14787: F, t14790: F, t14793: F, t14796: F, t14799: F, t974: F, t1716: F, t698: F, t3435: F, t4930: F, t1420: F, t1887: F, t337: F, t11593: F, t4904: F, t11570: F, t11569: F, t3452: F, t3472: F, t3478: F, t4899: F, t11571: F, t11545: F, t60: F, t461: F, t14726: F, t11589: F, t4729: F, t14736: F, t4900: F, t14740: F, t14731: F, t11575: F, t134: F, t3439: F, t4724: F, t11514: F, t11556: F, t11558: F, t11561: F, t225: F, t3507: F, t475: F, t6739: F, t1755: F, t11546: F, t15026: F, t3032: F, t3514: F, t3572: F, t5002: F, t3523: F, t5005: F, t5019: F, t5024: F, t11147: F, t11778: F, t14165: F, t4582: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15265, t15269, t15274, t15278, t15281) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1630::<F>(t1706, t3428, t1184, t460, t4928, t4934, t1714, t3469, t1178, t12606, t1177, t135, t457);
        let t15292 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1631::<F>(t15281, t4936, t1174, t3431, t4912, t1090, t7319, t4919, t11531, t11534, t11537, t11541, t11591, t15265, t15269, t15274, t15278, t3447);
        let (t15294, t15300, t15304, t15307) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1632::<F>(t11583, t3961, t3449, t11529, t1709, t1174, t1714, t3475, t460, t4934, t3432, t4889);
        let t15330 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1633::<F>(t3450, t3966, t3449, t14749, t4908, t3448, t4928, t3451, t11579, t4919, t11584, t1174, t15294, t15300, t15304, t15307, t3443, t3447, t3457, t3461, t4889);
        let (t15332, t15335, t15341, t15357) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1634::<F>(t14753, t4908, t14744, t11588, t1714, t3451, t3447, t14818, t14781, t14710, t11211, t11213, t11215, t11217, t11487, t14713, t14766, t14779, t14784, t14787, t14790, t14793, t14796, t14799);
        let (t15360, t15364, t15366, t15374, t15376) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1635::<F>(t15357, t457, t460, t974, t1716, t698, t1174, t3435, t4889, t135, t4930, t1420, t1887, t337);
        let t15386 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1636::<F>(t11593, t4904, t11570, t3961, t11569, t1174, t15332, t15335, t15341, t15360, t15364, t15366, t15374, t15376, t3447, t3452, t3472, t3478, t4889);
        let (t15391, t15396, t15401, t15403) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1637::<F>(t1714, t4899, t11571, t11545, t60, t461, t14726, t11589, t4904, t3447, t11588, t4729);
        let (t15405, t15406, t15409, t15412, t15415, t15420) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1638::<F>(t15403, t3447, t14736, t4900, t14740, t14731, t11575, t4904, t134, t3439, t461, t4724);
        let t15423 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1639::<F>(t15420, t3447, t11514, t11556, t11558, t11561, t15391, t15396, t15401, t15405, t15406, t15409, t15412, t15415);
        let (t15425, t15426, t15430, t15434, t15437) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1640::<F>(t15292, t15330, t15386, t15423, t225, t3507, t475, t6739, t1755, t11546, t14726, t15026, t3032);
        let (t15438, t15446, t15448, t15450, t15452, t15455) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1641::<F>(t15437, t3514, t3572, t5002, t3523, t5005, t5019, t5024, t11147, t11778, t14165, t4582);
    (t15425, t15426, t15430, t15434, t15437, t15438, t15446, t15448, t15450, t15452, t15455)
}
