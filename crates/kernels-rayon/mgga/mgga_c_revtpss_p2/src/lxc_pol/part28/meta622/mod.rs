//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2201;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2202;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2203;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2204;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2205;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2206;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta622(t4778: f64, t8521: f64, t1078: f64, t42859: f64, t1983: f64, t3143: f64, t11249: f64, t27641: f64, t1032: f64, t4930: f64, t994: f64, t15669: f64, t1976: f64, t1646: f64, t16561: f64, t16591: f64, t1695: f64, t25460: f64, t25473: f64, t25586: f64, t25591: f64, t25631: f64, t27427: f64, t27594: f64, t27598: f64, t27639: f64, t27643: f64, t27665: f64, t3046: f64, t3060: f64, t3075: f64, t3270: f64, t7144: f64, t7145: f64, t7147: f64, t7156: f64, t7159: f64, t7160: f64, t7817: f64, t7818: f64, t7828: f64, t93436: f64, t93498: f64, t93502: f64, t93904: f64, t93968: f64, t1668: f64, t7135: f64, t3153: f64, t4866: f64, t1035: f64, t73: f64, t3151: f64, t7821: f64, t1043: f64, t1089: f64, t1096: f64, t16568: f64, t16573: f64, t25461: f64, t25476: f64, t25601: f64, t25605: f64, t25611: f64, t27411: f64, t27422: f64, t27423: f64, t27426: f64, t27640: f64, t27642: f64, t27661: f64, t27664: f64, t27684: f64, t3133: f64, t3304: f64, t4910: f64, t4982: f64, t4997: f64, t4998: f64, t7151: f64, t93437: f64, t93890: f64, t93897: f64, t93983: f64, t94085: f64, t999: f64, t25698: f64, t93920: f64, t988: f64, t16237: f64, t16405: f64, t1982: f64, t1985: f64, t1986: f64, t25626: f64, t25629: f64, t27415: f64, t27444: f64, t27543: f64, t27595: f64, t27651: f64, t3042: f64, t3318: f64, t4763: f64, t4975: f64, t7810: f64, t7837: f64, t93921: f64, t94080: f64, t1647: f64, t3140: f64, t1097: f64, t15885: f64, t25464: f64, t25470: f64, t25588: f64, t25681: f64, t25699: f64, t27568: f64, t27576: f64, t27587: f64, t27609: f64, t27652: f64, t3059: f64, t3076: f64, t7167: f64, t7170: f64, t7174: f64, t7825: f64, t7829: f64, t94005: f64, t25604: f64, t1678: f64, t7150: f64, t27418: f64, t3057: f64, t1000: f64, t25593: f64, t25607: f64, t25613: f64, t25683: f64, t27433: f64, t27437: f64, t27621: f64, t27683: f64, t27687: f64, t7833: f64, t93497: f64, t93521: f64, t93939: f64, t93963: f64, t94042: f64, t94053: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99675, t99682, t99684, t99685, t99708, t99709, t99721) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2201(t4778, t8521, t1078, t42859, t1983, t3143, t11249, t27641, t1032, t4930, t994, t15669, t1976);
        let t99728 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2202(t1646, t16561, t16591, t1695, t1976, t25460, t25473, t25586, t25591, t25631, t27427, t27594, t27598, t27639, t27643, t27665, t3046, t3060, t3075, t3270, t7144, t7145, t7147, t7156, t7159, t7160, t7817, t7818, t7828, t93436, t93498, t93502, t93904, t93968, t99675, t99684, t99685, t99709, t99721);
        let (t99729, t99730, t99735, t99762, t99786, t99790) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2203(t1668, t7135, t3153, t1976, t4866, t1035, t1983, t99682, t73, t3151, t7821, t1043, t1089, t1096, t16568, t16573, t25461, t25476, t25601, t25605, t25611, t27411, t27422, t27423, t27426, t27640, t27642, t27661, t27664, t27684, t3133, t3304, t4910, t4982, t4997, t4998, t7144, t7151, t7160, t93437, t93890, t93897, t93983, t94085, t99685, t999);
        let t99847 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2204(t3151, t7817, t25698, t93920, t1096, t988, t1043, t1089, t16237, t16405, t1982, t1985, t1986, t25591, t25611, t25626, t25629, t27415, t27422, t27444, t27543, t27595, t27651, t3042, t3133, t3304, t3318, t4763, t4975, t7144, t7145, t7810, t7837, t93436, t93890, t93897, t93921, t94080, t99786, t999);
        let (t99877, t99901) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2205(t988, t999, t73, t99729, t1647, t7135, t1078, t1982, t3140, t4930, t1089, t1097, t15885, t1976, t25461, t25464, t25470, t25588, t25629, t25681, t25699, t27426, t27568, t27576, t27587, t27609, t27651, t27652, t3059, t3075, t3076, t3270, t4866, t4975, t7144, t7145, t7151, t7160, t7167, t7170, t7174, t7821, t7825, t7828, t7829, t93502, t94005);
        let t99950 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2206(t25604, t7825, t1678, t7150, t8521, t27418, t3057, t3046, t7810, t27543, t994, t1000, t1043, t1089, t1096, t1668, t25464, t25593, t25607, t25611, t25613, t25683, t27411, t27433, t27437, t27621, t27683, t27687, t3059, t7144, t7145, t7159, t7160, t7167, t7817, t7833, t93497, t93498, t93521, t93939, t93963, t94042, t94053, t988);
    (t99682, t99685, t99708, t99728, t99730, t99735, t99762, t99790, t99847, t99877, t99901, t99950)
}
