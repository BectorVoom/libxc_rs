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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta382<F: Float>(t15067: F, t3265: F, t11275: F, t14704: F, t14710: F, t14720: F, t11215: F, t11217: F, t14722: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F, t14766: F, t14781: F, t11137: F, t11139: F, t11141: F, t11143: F, t14728: F, t14809: F, t14811: F, t14814: F, t14816: F, t14818: F, t14824: F, t11211: F, t11213: F, t11314: F, t11317: F, t14702: F, t14708: F, t14713: F, t14759: F, t14779: F, t14784: F, t14787: F, t14790: F, t14793: F, t14796: F, t14799: F, t14802: F, t14805: F, t1137: F, t1147: F, t4832: F, t1687: F, t3400: F, t1156: F, t14829: F, t3375: F, t1129: F, t11356: F, t1148: F, t1157: F, t14840: F, t14847: F, t14849: F, t14852: F, t1695: F, t3371: F, t3378: F, t3396: F, t3404: F, t4835: F, t4858: F, t1128: F, t4794: F, t1675: F, t3356: F, t1136: F, t4820: F, t1683: F, t3351: F, t3333: F, t4823: F, t1138: F, t11410: F, t11420: F, t14864: F, t14866: F, t14916: F, t14934: F, t14939: F, t3327: F, t3332: F, t3352: F, t3360: F, t4797: F, t3359: F, t4819: F, t11352: F, t1682: F, t1155: F, t3395: F, t3377: F, t4861: F, t11444: F, t3331: F, t11297: F, t11350: F, t11361: F, t11365: F, t14958: F, t15048: F, t3334: F, t3357: F, t3376: F, t3401: F, t436: F, t4840: F, t4862: F, t3403: F, t4857: F, t11285: F, t1694: F, t11303: F, t11310: F, t11415: F, t15050: F, t15053: F, t15056: F, t15059: F, t15063: F, t15066: F, t4802: F, t4824: F, t300: F, t3411: F, t4875: F, t14963: F, t14969: F, t14971: F, t15038: F, t15040: F, t15043: F, t15046: F, t15035: F, t491: F, t1246: F, t15026: F, t3623: F, t11889: F, t3507: F, t1755: F, t15018: F, t3612: F, t5075: F, t5079: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15070, t15072, t15074, t15091) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1460::<F>(t15067, t3265, t11275, t14704, t14710, t14720, t11215, t11217, t14722, t14733, t14738, t14742, t14746, t14751, t14755, t14766);
        let (t15094, t15115) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1461::<F>(t14781, t11137, t11139, t11141, t11143, t14728, t14809, t14811, t14814, t14816, t14818, t14824);
        let t15117 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1462::<F>(t11211, t11213, t11314, t11317, t14702, t14708, t14713, t14759, t14779, t14784, t14787, t14790, t14793, t14796, t14799, t14802, t14805, t15072, t15074, t15091, t15094, t15115);
        let t15139 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1463::<F>(t1137, t15117, t1147, t4832, t1687, t3400, t1156, t14829, t3375, t1129, t11356, t1148, t1157, t14840, t14847, t14849, t14852, t1695, t3371, t3378, t3396, t3404, t4835, t4858);
        let t15162 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1464::<F>(t1128, t4794, t1675, t3356, t1136, t4820, t1683, t3351, t3333, t4823, t1138, t11410, t11420, t14864, t14866, t14916, t14934, t14939, t3327, t3332, t3352, t3360, t4797);
        let (t15165, t15168, t15172, t15179, t15182) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1465::<F>(t3359, t4819, t1136, t3351, t4823, t11352, t1682, t3333, t1155, t4858, t1695, t3395);
        let (t15185, t15204) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1466::<F>(t3377, t4861, t14722, t14704, t11137, t11139, t11141, t11143, t11444, t14702, t14708, t14720, t14728, t14733, t14738, t14742, t14746, t14751, t14755);
        let t15213 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1467::<F>(t1675, t3331, t1695, t3377, t11297, t11350, t11361, t11365, t14958, t15048, t15165, t15168, t15172, t15179, t15182, t15185, t15204, t3334, t3357, t3376, t3401, t436, t4840, t4862);
        let t15232 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1468::<F>(t3403, t4857, t1155, t3395, t4861, t11285, t1694, t3377, t1683, t3333, t11303, t11310, t11415, t15050, t15053, t15056, t15059, t15063, t15066, t15070, t3357, t3401, t4802, t4824);
        let (t15235, t15237, t15238) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1469::<F>(t15139, t15162, t15213, t15232, t300, t3411, t4875, t14958, t14963, t14969, t14971, t15038, t15040, t15043, t15046, t15048, t15050, t15053, t15056, t15059, t15063, t15066, t15070);
        let (t15239, t15241, t15245, t15248, t15253, t15257) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1470::<F>(t15035, t15238, t491, t1246, t15026, t3623, t11889, t3507, t1755, t15018, t3612, t5075, t5079);
    (t15070, t15235, t15237, t15239, t15241, t15245, t15248, t15253, t15257)
}
