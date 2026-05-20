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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta359<F: Float>(t14857: F, t2674: F, t243: F, t4423: F, t231: F, t2662: F, t2661: F, t10722: F, t1565: F, t4352: F, t4366: F, t10726: F, t2430: F, t2747: F, t4365: F, t10762: F, t10783: F, t10812: F, t10816: F, t10900: F, t14843: F, t14846: F, t14850: F, t14853: F, t2745: F, t851: F, t10824: F, t10826: F, t10833: F, t10838: F, t10842: F, t10846: F, t10853: F, t10855: F, t10859: F, t10881: F, t10885: F, t10888: F, t10868: F, t241: F, t820: F, t14547: F, t4364: F, t2724: F, t4450: F, t14676: F, t10811: F, t4452: F, t2754: F, t2394: F, t10770: F, t2719: F, t844: F, t4368: F, t2482: F, t814: F, t14671: F, t14686: F, t10891: F, t10893: F, t10906: F, t4362: F, t14711: F, t14754: F, t14784: F, t14811: F, t14841: F, t136: F, t1568: F, t2457: F, t2710: F, t2470: F, t4522: F, t874: F, t10657: F, t10916: F, t10921: F, t14577: F, t14581: F, t14590: F, t14596: F, t14603: F, t14608: F, t14663: F, t1559: F, t213: F, t234: F, t2815: F, t4424: F, t4494: F, t4514: F, t879: F, t2718: F, t4469: F, t822: F, t10923: F, t10925: F, t10930: F, t10935: F, t10939: F, t10948: F, t10961: F, t10964: F, t10966: F, t10969: F, t10971: F, t10974: F, t14507: F, t2646: F, t4526: F, t837: F, t14540: F, t14572: F, t868: F, t4533: F, t72: F, t686: F, t2465: F, t1569: F, t867: F, t786: F, t2467: F, t122: F, t4480: F, t2466: F, t10995: F, t11044: F, t4481: F, t10498: F, t10501: F, t14474: F, t14479: F, t14484: F, t14486: F, t14489: F, t865: F, t2435: F, t4477: F, t1579: F, t10504: F, t2471: F, t4325: F, t1580: F, t2444: F, t689: F) -> (F, F, F, F, F, F, F) {
        let (t14859, t14864, t14866, t14869) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1243::<F>(t14857, t2674, t243, t4423, t231, t2662, t2661, t10722, t1565, t4352, t4366, t10726);
        let t14878 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1244::<F>(t14869, t2661, t231, t2430, t2747, t4365, t10762, t10783, t10812, t10816, t10900, t14843, t14846, t14850, t14853, t14859, t14864, t14866, t2745, t851);
        let t14889 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1245::<F>(t10824, t10826, t10833, t10838, t10842, t10846, t10853, t10855, t10859, t10881, t10885, t10888);
        let (t14894, t14896, t14900, t14904, t14907) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1246::<F>(t10868, t241, t820, t14547, t4364, t4365, t2724, t2747, t4450, t14676, t4366, t10811, t4452);
        let (t14910, t14914, t14919, t14925) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1247::<F>(t2747, t2754, t4450, t4364, t4365, t231, t2394, t10770, t2719, t820, t844, t4368);
        let t14936 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1248::<F>(t2724, t4364, t4365, t2482, t2719, t814, t14671, t14686, t4366, t10891, t10893, t10906, t14894, t14896, t14900, t14904, t14907, t14910, t14914, t14919, t14925, t2745, t4362);
        let (t14939, t14948) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1249::<F>(t14711, t14754, t14784, t14811, t14841, t14878, t14889, t14936, t136, t1568, t2457, t2710);
        let t14953 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1250::<F>(t2470, t4522, t874, t10657, t10916, t10921, t14577, t14581, t14590, t14596, t14603, t14608, t14663, t14939, t14948, t1559, t213, t234, t2754, t2815, t4424, t4494, t4514, t820, t879);
        let t14976 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1251::<F>(t1568, t2718, t4469, t822, t10923, t10925, t10930, t10935, t10939, t10948, t10961, t10964, t10966, t10969, t10971, t10974, t14507, t2646, t2724, t4514, t4526, t820, t837);
        let (t14979, t14985, t14987) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1252::<F>(t14540, t14572, t14953, t14976, t868, t4533, t72, t686, t2465, t1569, t867, t786);
        let t14997 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1253::<F>(t14987, t2467, t122, t4480, t2466, t10995, t11044, t4481, t10498, t10501, t14474, t14479, t14484, t14486, t14489, t14979, t14985, t865);
        let (t14998, t15004, t15006, t15010, t15011) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1254::<F>(t2435, t4477, t136, t1579, t2457, t10504, t2471, t4325, t1580, t2444, t689, t213, t4469);
    (t14939, t14997, t14998, t15004, t15006, t15010, t15011)
}
