//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta359 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1243;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1244;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1245;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1246;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1247;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1248;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1249;
use chunk7::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1250;
use chunk8::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1251;
use chunk9::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1252;
use chunk10::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1253;
use chunk11::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1254;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta359(t14857: f64, t2674: f64, t243: f64, t4423: f64, t231: f64, t2662: f64, t2661: f64, t10722: f64, t1565: f64, t4352: f64, t4366: f64, t10726: f64, t2430: f64, t2747: f64, t4365: f64, t10762: f64, t10783: f64, t10812: f64, t10816: f64, t10900: f64, t14843: f64, t14846: f64, t14850: f64, t14853: f64, t2745: f64, t851: f64, t10824: f64, t10826: f64, t10833: f64, t10838: f64, t10842: f64, t10846: f64, t10853: f64, t10855: f64, t10859: f64, t10881: f64, t10885: f64, t10888: f64, t10868: f64, t241: f64, t820: f64, t14547: f64, t4364: f64, t2724: f64, t4450: f64, t14676: f64, t10811: f64, t4452: f64, t2754: f64, t2394: f64, t10770: f64, t2719: f64, t844: f64, t4368: f64, t2482: f64, t814: f64, t14671: f64, t14686: f64, t10891: f64, t10893: f64, t10906: f64, t4362: f64, t14711: f64, t14754: f64, t14784: f64, t14811: f64, t14841: f64, t136: f64, t1568: f64, t2457: f64, t2710: f64, t2470: f64, t4522: f64, t874: f64, t10657: f64, t10916: f64, t10921: f64, t14577: f64, t14581: f64, t14590: f64, t14596: f64, t14603: f64, t14608: f64, t14663: f64, t1559: f64, t213: f64, t234: f64, t2815: f64, t4424: f64, t4494: f64, t4514: f64, t879: f64, t2718: f64, t4469: f64, t822: f64, t10923: f64, t10925: f64, t10930: f64, t10935: f64, t10939: f64, t10948: f64, t10961: f64, t10964: f64, t10966: f64, t10969: f64, t10971: f64, t10974: f64, t14507: f64, t2646: f64, t4526: f64, t837: f64, t14540: f64, t14572: f64, t868: f64, t4533: f64, t72: f64, t686: f64, t2465: f64, t1569: f64, t867: f64, t786: f64, t2467: f64, t122: f64, t4480: f64, t2466: f64, t10995: f64, t11044: f64, t4481: f64, t10498: f64, t10501: f64, t14474: f64, t14479: f64, t14484: f64, t14486: f64, t14489: f64, t865: f64, t2435: f64, t4477: f64, t1579: f64, t10504: f64, t2471: f64, t4325: f64, t1580: f64, t2444: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t14859, t14864, t14866, t14869) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1243(t14857, t2674, t243, t4423, t231, t2662, t2661, t10722, t1565, t4352, t4366, t10726);
        let t14878 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1244(t14869, t2661, t231, t2430, t2747, t4365, t10762, t10783, t10812, t10816, t10900, t14843, t14846, t14850, t14853, t14859, t14864, t14866, t2745, t851);
        let t14889 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1245(t10824, t10826, t10833, t10838, t10842, t10846, t10853, t10855, t10859, t10881, t10885, t10888);
        let (t14894, t14896, t14900, t14904, t14907) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1246(t10868, t241, t820, t14547, t4364, t4365, t2724, t2747, t4450, t14676, t4366, t10811, t4452);
        let (t14910, t14914, t14919, t14925) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1247(t2747, t2754, t4450, t4364, t4365, t231, t2394, t10770, t2719, t820, t844, t4368);
        let t14936 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1248(t2724, t4364, t4365, t2482, t2719, t814, t14671, t14686, t4366, t10891, t10893, t10906, t14894, t14896, t14900, t14904, t14907, t14910, t14914, t14919, t14925, t2745, t4362);
        let (t14939, t14948) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1249(t14711, t14754, t14784, t14811, t14841, t14878, t14889, t14936, t136, t1568, t2457, t2710);
        let t14953 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1250(t2470, t4522, t874, t10657, t10916, t10921, t14577, t14581, t14590, t14596, t14603, t14608, t14663, t14939, t14948, t1559, t213, t234, t2754, t2815, t4424, t4494, t4514, t820, t879);
        let t14976 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1251(t1568, t2718, t4469, t822, t10923, t10925, t10930, t10935, t10939, t10948, t10961, t10964, t10966, t10969, t10971, t10974, t14507, t2646, t2724, t4514, t4526, t820, t837);
        let (t14979, t14985, t14987) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1252(t14540, t14572, t14953, t14976, t868, t4533, t72, t686, t2465, t1569, t867, t786);
        let t14997 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1253(t14987, t2467, t122, t4480, t2466, t10995, t11044, t4481, t10498, t10501, t14474, t14479, t14484, t14486, t14489, t14979, t14985, t865);
        let (t14998, t15004, t15006, t15010, t15011) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1254(t2435, t4477, t136, t1579, t2457, t10504, t2471, t4325, t1580, t2444, t689, t213, t4469);
    (t14939, t14997, t14998, t15004, t15006, t15010, t15011)
}
