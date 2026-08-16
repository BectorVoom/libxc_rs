//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta396 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1622;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1623;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1624;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1625;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1626;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1627;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1628;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1629;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta396(t1706: f64, t3428: f64, t1184: f64, t460: f64, t4928: f64, t4934: f64, t1714: f64, t3469: f64, t1178: f64, t12606: f64, t1177: f64, t135: f64, t457: f64, t4936: f64, t1174: f64, t3431: f64, t4912: f64, t1090: f64, t7319: f64, t4919: f64, t11531: f64, t11534: f64, t11537: f64, t11541: f64, t11591: f64, t3447: f64, t11583: f64, t3961: f64, t3449: f64, t11529: f64, t1709: f64, t3475: f64, t3432: f64, t4889: f64, t3450: f64, t3966: f64, t14749: f64, t4908: f64, t3448: f64, t3451: f64, t11579: f64, t11584: f64, t3443: f64, t3457: f64, t3461: f64, t14753: f64, t14744: f64, t11588: f64, t14818: f64, t14781: f64, t14710: f64, t11211: f64, t11213: f64, t11215: f64, t11217: f64, t11487: f64, t14713: f64, t14766: f64, t14779: f64, t14784: f64, t14787: f64, t14790: f64, t14793: f64, t14796: f64, t14799: f64, t974: f64, t1716: f64, t698: f64, t3435: f64, t4930: f64, t1420: f64, t1887: f64, t337: f64, t11593: f64, t4904: f64, t11570: f64, t11569: f64, t3452: f64, t3472: f64, t3478: f64, t4899: f64, t11571: f64, t11545: f64, t60: f64, t461: f64, t14726: f64, t11589: f64, t4729: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15265, t15269, t15274, t15278, t15281) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1622(t1706, t3428, t1184, t460, t4928, t4934, t1714, t3469, t1178, t12606, t1177, t135, t457);
        let t15292 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1623(t15281, t4936, t1174, t3431, t4912, t1090, t7319, t4919, t11531, t11534, t11537, t11541, t11591, t15265, t15269, t15274, t15278, t3447);
        let (t15294, t15300, t15304, t15307) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1624(t11583, t3961, t3449, t11529, t1709, t1174, t1714, t3475, t460, t4934, t3432, t4889);
        let t15330 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1625(t3450, t3966, t3449, t14749, t4908, t3448, t4928, t3451, t11579, t4919, t11584, t1174, t15294, t15300, t15304, t15307, t3443, t3447, t3457, t3461, t4889);
        let (t15332, t15335, t15341, t15357) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1626(t14753, t4908, t14744, t11588, t1714, t3451, t3447, t14818, t14781, t14710, t11211, t11213, t11215, t11217, t11487, t14713, t14766, t14779, t14784, t14787, t14790, t14793, t14796, t14799);
        let (t15359, t15360, t15364, t15366, t15374, t15376) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1627(t15357, t457, t460, t974, t1716, t698, t1174, t3435, t4889, t135, t4930, t1420, t1887, t337);
        let t15386 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1628(t11593, t4904, t11570, t3961, t11569, t1174, t15332, t15335, t15341, t15360, t15364, t15366, t15374, t15376, t3447, t3452, t3472, t3478, t4889);
        let (t15391, t15394, t15396, t15401, t15403) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1629(t1714, t4899, t11571, t11545, t60, t461, t14726, t11589, t4904, t3447, t11588, t4729);
    (t15292, t15330, t15357, t15359, t15386, t15391, t15394, t15396, t15401, t15403)
}
