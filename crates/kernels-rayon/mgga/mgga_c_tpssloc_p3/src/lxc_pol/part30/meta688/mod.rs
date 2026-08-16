//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta688 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2185;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2186;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2187;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2188;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2189;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2190;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2191;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2192;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2193;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2194;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta688(t28200: f64, t6883: f64, t225: f64, t28053: f64, t6888: f64, t7691: f64, t90739: f64, t1375: f64, t1386: f64, t20025: f64, t2016: f64, t26224: f64, t26225: f64, t26366: f64, t3887: f64, t5210: f64, t5354: f64, t539: f64, t56422: f64, t568: f64, t6460: f64, t6992: f64, t7722: f64, t81399: f64, t93906: f64, t97468: f64, t12020: f64, t1378: f64, t1390: f64, t16022: f64, t16030: f64, t16439: f64, t1807: f64, t1843: f64, t19648: f64, t1983: f64, t20022: f64, t20023: f64, t20029: f64, t20051: f64, t20060: f64, t2015: f64, t26226: f64, t26328: f64, t26348: f64, t26371: f64, t28111: f64, t28220: f64, t28224: f64, t3758: f64, t3882: f64, t5215: f64, t5321: f64, t5325: f64, t5326: f64, t533: f64, t56434: f64, t56580: f64, t56596: f64, t56607: f64, t6958: f64, t6963: f64, t7729: f64, t7749: f64, t7750: f64, t80711: f64, t81267: f64, t81282: f64, t81318: f64, t81375: f64, t90512: f64, t90515: f64, t90521: f64, t90585: f64, t90687: f64, t91441: f64, t91488: f64, t93335: f64, t93368: f64, t93387: f64, t93446: f64, t93899: f64, t96885: f64, t96893: f64, t96896: f64, t96900: f64, t96917: f64, t96920: f64, t96925: f64, t96929: f64, t96960: f64, t96999: f64, t97032: f64, t97075: f64, t97116: f64, t97154: f64, t97196: f64, t97496: f64, t97503: f64, t97519: f64, t97524: f64, t97527: f64, t97529: f64, t97552: f64, t97607: f64, t97611: f64, t97616: f64, t97619: f64, t97624: f64, t97626: f64, t97666: f64, t97717: f64, t97724: f64, t97729: f64, t97732: f64, t97740: f64, t24987: f64, t7756: f64, t2314: f64, t28025: f64, t4034: f64, t1266: f64, t28017: f64, t652: f64, t1845: f64, t5187: f64, t22574: f64, t8643: f64, t7688: f64, t1874: f64, t75560: f64, t19451: f64, t6525: f64, t25994: f64, t4028: f64, t55943: f64, t191: f64, t192: f64, t19537: f64, t2020: f64, t15868: f64, t7753: f64, t74032: f64, t24999: f64, t4073: f64, t5361: f64, t7681: f64, t96842: f64, t96844: f64, t96846: f64, t28237: f64, t532: f64, t6879: f64, t510: f64, t96729: f64, t96683: f64, t25992: f64, t7685: f64, t25985: f64, t28821: f64, t7000: f64, t24990: f64, t26167: f64, t7687: f64, t91620: f64, t28002: f64, t6535: f64, t12725: f64, t7461: f64, t19456: f64, t25980: f64, t7468: f64, t28045: f64, t24983: f64, t25965: f64, t7472: f64, t5107: f64, t7467: f64, t1774: f64, t26135: f64, t26179: f64, t7458: f64, t28826: f64, t31299: f64, t1388: f64, t6324: f64, t26161: f64, t91686: f64, t26504: f64, t22591: f64, t28834: f64, t19596: f64, t6996: f64, t24994: f64, t7684: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t97770 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2185(t28200, t6883, t225, t28053, t6888, t7691, t90739, t1375, t1386, t20025, t2016, t26224, t26225, t26366, t3887, t5210, t5354, t539, t56422, t568, t6460, t6992, t7722, t81399, t93906, t97468);
        let t97777 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2186(t12020, t1375, t1378, t1390, t16022, t16030, t16439, t1807, t1843, t19648, t1983, t20022, t20023, t20029, t20051, t20060, t2015, t2016, t26224, t26226, t26328, t26348, t26366, t26371, t28111, t28220, t28224, t3758, t3882, t3887, t5215, t5321, t5325, t5326, t533, t56434, t56580, t56596, t56607, t568, t6958, t6963, t7729, t7749, t7750, t80711, t81267, t81282, t81318, t81375, t90512, t90515, t90521, t90585, t90687, t91441, t91488, t93335, t93368, t93387, t93446, t93899, t96885, t96893, t96896, t96900, t96917, t96920, t96925, t96929, t96960, t96999, t97032, t97075, t97116, t97154, t97196, t97496, t97503, t97519, t97524, t97527, t97529, t97552, t97607, t97611, t97616, t97619, t97624, t97626, t97666, t97717, t97724, t97729, t97732, t97740, t97770);
        let (t97779, t97783, t97785, t97788, t97792) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2187(t24987, t7756, t2314, t28025, t4034, t1266, t28017, t652, t1845, t5187, t22574, t8643);
        let (t97794, t97796, t97798, t97800, t97802, t97804) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2188(t24987, t7688, t1874, t75560, t19451, t6525, t25994, t4028, t55943, t191, t192, t19537);
        let t97814 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2189(t2020, t97804, t15868, t1983, t7753, t22574, t74032, t8643, t24999, t4073, t5361, t7681, t96842, t96844, t96846, t97777, t97779, t97783, t97785, t97788, t97792, t97794, t97796, t97798, t97800, t97802);
        let (t97820, t97829, t97831, t97833, t97835) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2190(t28237, t532, t1983, t6879, t510, t652, t96729, t1874, t96683, t25992, t7685, t25985);
        let (t97836, t97839, t97842, t97844, t97846, t97848) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2191(t28821, t7000, t1983, t24990, t26167, t7687, t91620, t28002, t6535, t12725, t7461, t19456);
        let t97859 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2192(t25980, t4028, t12725, t7468, t2314, t28045, t4034, t19456, t24983, t25965, t7472, t97820, t97829, t97831, t97833, t97835, t97836, t97839, t97842, t97844, t97846, t97848);
        let (t97862, t97865, t97869, t97871, t97874) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2193(t5107, t652, t7467, t1774, t26135, t26179, t7461, t25980, t7458, t1983, t28826, t31299);
        let (t97878, t97880, t97887, t97889, t97890) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2194(t1388, t6324, t26161, t91686, t26504, t7685, t1983, t22591, t28834, t19596, t6996, t24994, t7684);
    (t97814, t97859, t97862, t97865, t97869, t97871, t97874, t97878, t97880, t97887, t97889, t97890)
}
