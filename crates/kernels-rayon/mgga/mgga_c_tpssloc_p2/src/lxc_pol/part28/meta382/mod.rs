//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta382 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1460;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1461;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1462;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1463;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1464;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1465;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1466;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1467;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1468;
use chunk9::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1469;
use chunk10::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta382(t15067: f64, t3265: f64, t11275: f64, t14704: f64, t14710: f64, t14720: f64, t11215: f64, t11217: f64, t14722: f64, t14733: f64, t14738: f64, t14742: f64, t14746: f64, t14751: f64, t14755: f64, t14766: f64, t14781: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t14728: f64, t14809: f64, t14811: f64, t14814: f64, t14816: f64, t14818: f64, t14824: f64, t11211: f64, t11213: f64, t11314: f64, t11317: f64, t14702: f64, t14708: f64, t14713: f64, t14759: f64, t14779: f64, t14784: f64, t14787: f64, t14790: f64, t14793: f64, t14796: f64, t14799: f64, t14802: f64, t14805: f64, t1137: f64, t1147: f64, t4832: f64, t1687: f64, t3400: f64, t1156: f64, t14829: f64, t3375: f64, t1129: f64, t11356: f64, t1148: f64, t1157: f64, t14840: f64, t14847: f64, t14849: f64, t14852: f64, t1695: f64, t3371: f64, t3378: f64, t3396: f64, t3404: f64, t4835: f64, t4858: f64, t1128: f64, t4794: f64, t1675: f64, t3356: f64, t1136: f64, t4820: f64, t1683: f64, t3351: f64, t3333: f64, t4823: f64, t1138: f64, t11410: f64, t11420: f64, t14864: f64, t14866: f64, t14916: f64, t14934: f64, t14939: f64, t3327: f64, t3332: f64, t3352: f64, t3360: f64, t4797: f64, t3359: f64, t4819: f64, t11352: f64, t1682: f64, t1155: f64, t3395: f64, t3377: f64, t4861: f64, t11444: f64, t3331: f64, t11297: f64, t11350: f64, t11361: f64, t11365: f64, t14958: f64, t15048: f64, t3334: f64, t3357: f64, t3376: f64, t3401: f64, t436: f64, t4840: f64, t4862: f64, t3403: f64, t4857: f64, t11285: f64, t1694: f64, t11303: f64, t11310: f64, t11415: f64, t15050: f64, t15053: f64, t15056: f64, t15059: f64, t15063: f64, t15066: f64, t4802: f64, t4824: f64, t300: f64, t3411: f64, t4875: f64, t14963: f64, t14969: f64, t14971: f64, t15038: f64, t15040: f64, t15043: f64, t15046: f64, t15035: f64, t491: f64, t1246: f64, t15026: f64, t3623: f64, t11889: f64, t3507: f64, t1755: f64, t15018: f64, t3612: f64, t5075: f64, t5079: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15070, t15072, t15074, t15091) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1460(t15067, t3265, t11275, t14704, t14710, t14720, t11215, t11217, t14722, t14733, t14738, t14742, t14746, t14751, t14755, t14766);
        let (t15094, t15115) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1461(t14781, t11137, t11139, t11141, t11143, t14728, t14809, t14811, t14814, t14816, t14818, t14824);
        let t15117 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1462(t11211, t11213, t11314, t11317, t14702, t14708, t14713, t14759, t14779, t14784, t14787, t14790, t14793, t14796, t14799, t14802, t14805, t15072, t15074, t15091, t15094, t15115);
        let t15139 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1463(t1137, t15117, t1147, t4832, t1687, t3400, t1156, t14829, t3375, t1129, t11356, t1148, t1157, t14840, t14847, t14849, t14852, t1695, t3371, t3378, t3396, t3404, t4835, t4858);
        let t15162 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1464(t1128, t4794, t1675, t3356, t1136, t4820, t1683, t3351, t3333, t4823, t1138, t11410, t11420, t14864, t14866, t14916, t14934, t14939, t3327, t3332, t3352, t3360, t4797);
        let (t15165, t15168, t15172, t15179, t15182) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1465(t3359, t4819, t1136, t3351, t4823, t11352, t1682, t3333, t1155, t4858, t1695, t3395);
        let (t15185, t15204) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1466(t3377, t4861, t14722, t14704, t11137, t11139, t11141, t11143, t11444, t14702, t14708, t14720, t14728, t14733, t14738, t14742, t14746, t14751, t14755);
        let t15213 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1467(t1675, t3331, t1695, t3377, t11297, t11350, t11361, t11365, t14958, t15048, t15165, t15168, t15172, t15179, t15182, t15185, t15204, t3334, t3357, t3376, t3401, t436, t4840, t4862);
        let t15232 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1468(t3403, t4857, t1155, t3395, t4861, t11285, t1694, t3377, t1683, t3333, t11303, t11310, t11415, t15050, t15053, t15056, t15059, t15063, t15066, t15070, t3357, t3401, t4802, t4824);
        let (t15235, t15237, t15238) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1469(t15139, t15162, t15213, t15232, t300, t3411, t4875, t14958, t14963, t14969, t14971, t15038, t15040, t15043, t15046, t15048, t15050, t15053, t15056, t15059, t15063, t15066, t15070);
        let (t15239, t15241, t15245, t15248, t15253, t15257) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1470(t15035, t15238, t491, t1246, t15026, t3623, t11889, t3507, t1755, t15018, t3612, t5075, t5079);
    (t15070, t15235, t15237, t15239, t15241, t15245, t15248, t15253, t15257)
}
