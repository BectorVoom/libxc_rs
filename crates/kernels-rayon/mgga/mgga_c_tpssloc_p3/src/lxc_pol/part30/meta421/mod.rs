//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta421 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1609;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1610;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1611;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1612;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1613;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1614;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1615;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1616;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1617;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1618;
use chunk10::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1619;
use chunk11::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1620;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta421(t15453: f64, t17686: f64, t4582: f64, t17635: f64, t4972: f64, t1090: f64, t6230: f64, t3578: f64, t6219: f64, t4997: f64, t5002: f64, t11784: f64, t248: f64, t5971: f64, t1227: f64, t5019: f64, t4993: f64, t5005: f64, t1202: f64, t6164: f64, t5024: f64, t11692: f64, t11792: f64, t11821: f64, t15671: f64, t15691: f64, t15699: f64, t15740: f64, t3577: f64, t488: f64, t4950: f64, t1196: f64, t16558: f64, t974: f64, t1215: f64, t1653: f64, t15659: f64, t1177: f64, t18221: f64, t18237: f64, t1735: f64, t4724: f64, t11668: f64, t18232: f64, t3440: f64, t1017: f64, t6163: f64, t1210: f64, t1207: f64, t11665: f64, t11678: f64, t1174: f64, t11834: f64, t1218: f64, t15569: f64, t15717: f64, t15719: f64, t15722: f64, t4889: f64, t4954: f64, t4969: f64, t5046: f64, t6192: f64, t372: f64, t479: f64, t471: f64, t3521: f64, t5979: f64, t1009: f64, t6150: f64, t1011: f64, t1212: f64, t1226: f64, t6169: f64, t486: f64, t6218: f64, t4978: f64, t1216: f64, t4987: f64, t4977: f64, t5012: f64, t11836: f64, t1232: f64, t15495: f64, t15727: f64, t15731: f64, t15735: f64, t15745: f64, t1737: f64, t3506: f64, t3515: f64, t3536: f64, t4989: f64, t6221: f64, t18300: f64, t5001: f64, t5018: f64, t1730: f64, t5023: f64, t18225: f64, t1193: f64, t6109: f64, t3570: f64, t1230: f64, t18241: f64, t11546: f64, t18206: f64, t11738: f64, t15591: f64, t15594: f64, t15754: f64, t1748: f64, t3490: f64, t5014: f64, t5030: f64, t5033: f64, t6207: f64, t6211: f64, t18316: f64, t18337: f64, t18390: f64, t18951: f64, t466: f64, t5068: f64, t6260: f64, t18940: f64, t491: f64, t1246: f64, t5079: f64, t6256: f64, t3625: f64, t5011: f64, t1755: f64, t1235: f64, t6224: f64, t475: f64, t6739: f64, t6252: f64, t11889: f64, t11888: f64, t11904: f64, t11907: f64, t11914: f64, t1244: f64, t15027: f64, t15032: f64, t15245: f64, t1756: f64, t3604: f64, t3610: f64, t3624: f64, t5064: f64, t5069: f64, t5080: f64, t5084: f64, t6253: f64, t6261: f64, t6263: f64, t11883: f64, t1751: f64, t6238: f64, t3612: f64, t1734: f64, t5052: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18955, t18959, t18965, t18969, t18972, t18975) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1609(t15453, t17686, t4582, t17635, t4972, t1090, t6230, t3578, t6219, t4997, t5002, t11784, t248, t5971);
        let t18989 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1610(t1227, t18975, t4997, t5019, t4993, t5005, t1202, t6164, t5024, t11692, t11792, t11821, t15671, t15691, t15699, t15740, t18955, t18959, t18965, t18969, t18972, t3577, t488, t4950);
        let (t18997, t19002, t19005, t19010, t19015) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1611(t1196, t16558, t974, t1215, t1653, t15659, t3578, t1177, t18221, t18237, t1735, t4724);
        let t19029 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1612(t11668, t19015, t18232, t3440, t1017, t6163, t1210, t1207, t11665, t11678, t1174, t11834, t1218, t15569, t15717, t15719, t15722, t15740, t18997, t19002, t19005, t19010, t3577, t4889, t4950, t4954, t4969, t5046, t6192);
        let (t19033, t19041, t19045, t19047) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1613(t372, t6163, t479, t471, t248, t3521, t5979, t1227, t1009, t6150, t1011, t1212);
        let t19075 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1614(t1226, t6169, t486, t6218, t4978, t4582, t1216, t17635, t4987, t4977, t5012, t11836, t1218, t1227, t1232, t15495, t15727, t15731, t15735, t15745, t1737, t19033, t19041, t19047, t3506, t3515, t3536, t4989, t5024, t6221);
        let (t19077, t19080, t19083, t19087, t19090, t19095) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1615(t1216, t18300, t4582, t5001, t5018, t1730, t5023, t1177, t18225, t1193, t6109, t248, t3570, t6230);
        let t19117 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1616(t19095, t3515, t1230, t18241, t248, t11546, t18206, t11738, t1174, t1218, t1227, t1232, t15591, t15594, t15754, t1737, t1748, t19077, t19080, t19083, t19087, t19090, t3490, t4889, t5002, t5005, t5014, t5030, t5033, t6207, t6211);
        let (t19120, t19121, t19123, t19128) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1617(t18316, t18337, t18390, t18951, t18989, t19029, t19075, t19117, t466, t5068, t6260, t18940, t491);
        let (t19129, t19131, t19139, t19142, t19145, t19146, t19153) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1618(t1246, t19128, t5079, t6256, t3625, t5011, t1755, t5068, t1235, t6224, t1215, t475, t6739);
        let t19164 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1619(t19153, t6252, t11889, t1215, t5079, t6260, t11888, t11904, t11907, t11914, t1244, t15027, t15032, t15245, t1756, t19123, t19129, t19131, t19139, t19142, t19146, t3604, t3610, t3624, t5064, t5069, t5080, t5084, t6253, t6261, t6263);
        let (t19166, t19170, t19174, t19176, t19180, t19189) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1620(t11883, t1215, t6252, t1751, t5011, t1246, t6238, t19145, t3612, t1734, t5052, t1235, t6218);
    (t19045, t19120, t19121, t19164, t19166, t19170, t19174, t19176, t19180, t19189)
}
