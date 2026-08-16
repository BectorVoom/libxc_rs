//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2201;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2202;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2203;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2204;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2205;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2206;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta622<F: Float>(t4778: F, t8521: F, t1078: F, t42859: F, t1983: F, t3143: F, t11249: F, t27641: F, t1032: F, t4930: F, t994: F, t15669: F, t1976: F, t1646: F, t16561: F, t16591: F, t1695: F, t25460: F, t25473: F, t25586: F, t25591: F, t25631: F, t27427: F, t27594: F, t27598: F, t27639: F, t27643: F, t27665: F, t3046: F, t3060: F, t3075: F, t3270: F, t7144: F, t7145: F, t7147: F, t7156: F, t7159: F, t7160: F, t7817: F, t7818: F, t7828: F, t93436: F, t93498: F, t93502: F, t93904: F, t93968: F, t1668: F, t7135: F, t3153: F, t4866: F, t1035: F, t73: F, t3151: F, t7821: F, t1043: F, t1089: F, t1096: F, t16568: F, t16573: F, t25461: F, t25476: F, t25601: F, t25605: F, t25611: F, t27411: F, t27422: F, t27423: F, t27426: F, t27640: F, t27642: F, t27661: F, t27664: F, t27684: F, t3133: F, t3304: F, t4910: F, t4982: F, t4997: F, t4998: F, t7151: F, t93437: F, t93890: F, t93897: F, t93983: F, t94085: F, t999: F, t25698: F, t93920: F, t988: F, t16237: F, t16405: F, t1982: F, t1985: F, t1986: F, t25626: F, t25629: F, t27415: F, t27444: F, t27543: F, t27595: F, t27651: F, t3042: F, t3318: F, t4763: F, t4975: F, t7810: F, t7837: F, t93921: F, t94080: F, t1647: F, t3140: F, t1097: F, t15885: F, t25464: F, t25470: F, t25588: F, t25681: F, t25699: F, t27568: F, t27576: F, t27587: F, t27609: F, t27652: F, t3059: F, t3076: F, t7167: F, t7170: F, t7174: F, t7825: F, t7829: F, t94005: F, t25604: F, t1678: F, t7150: F, t27418: F, t3057: F, t1000: F, t25593: F, t25607: F, t25613: F, t25683: F, t27433: F, t27437: F, t27621: F, t27683: F, t27687: F, t7833: F, t93497: F, t93521: F, t93939: F, t93963: F, t94042: F, t94053: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t99675, t99682, t99684, t99685, t99708, t99709, t99721) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2201::<F>(t4778, t8521, t1078, t42859, t1983, t3143, t11249, t27641, t1032, t4930, t994, t15669, t1976);
        let t99728 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2202::<F>(t1646, t16561, t16591, t1695, t1976, t25460, t25473, t25586, t25591, t25631, t27427, t27594, t27598, t27639, t27643, t27665, t3046, t3060, t3075, t3270, t7144, t7145, t7147, t7156, t7159, t7160, t7817, t7818, t7828, t93436, t93498, t93502, t93904, t93968, t99675, t99684, t99685, t99709, t99721);
        let (t99729, t99730, t99735, t99762, t99786, t99790) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2203::<F>(t1668, t7135, t3153, t1976, t4866, t1035, t1983, t99682, t73, t3151, t7821, t1043, t1089, t1096, t16568, t16573, t25461, t25476, t25601, t25605, t25611, t27411, t27422, t27423, t27426, t27640, t27642, t27661, t27664, t27684, t3133, t3304, t4910, t4982, t4997, t4998, t7144, t7151, t7160, t93437, t93890, t93897, t93983, t94085, t99685, t999);
        let t99847 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2204::<F>(t3151, t7817, t25698, t93920, t1096, t988, t1043, t1089, t16237, t16405, t1982, t1985, t1986, t25591, t25611, t25626, t25629, t27415, t27422, t27444, t27543, t27595, t27651, t3042, t3133, t3304, t3318, t4763, t4975, t7144, t7145, t7810, t7837, t93436, t93890, t93897, t93921, t94080, t99786, t999);
        let (t99877, t99901) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2205::<F>(t988, t999, t73, t99729, t1647, t7135, t1078, t1982, t3140, t4930, t1089, t1097, t15885, t1976, t25461, t25464, t25470, t25588, t25629, t25681, t25699, t27426, t27568, t27576, t27587, t27609, t27651, t27652, t3059, t3075, t3076, t3270, t4866, t4975, t7144, t7145, t7151, t7160, t7167, t7170, t7174, t7821, t7825, t7828, t7829, t93502, t94005);
        let t99950 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2206::<F>(t25604, t7825, t1678, t7150, t8521, t27418, t3057, t3046, t7810, t27543, t994, t1000, t1043, t1089, t1096, t1668, t25464, t25593, t25607, t25611, t25613, t25683, t27411, t27433, t27437, t27621, t27683, t27687, t3059, t7144, t7145, t7159, t7160, t7167, t7817, t7833, t93497, t93498, t93521, t93939, t93963, t94042, t94053, t988);
    (t99682, t99685, t99708, t99728, t99730, t99735, t99762, t99790, t99847, t99877, t99901, t99950)
}
