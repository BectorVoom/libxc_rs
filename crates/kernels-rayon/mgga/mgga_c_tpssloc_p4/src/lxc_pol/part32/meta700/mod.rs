//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta700 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2193;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2194;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2195;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2196;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta700(t28200: f64, t6883: f64, t225: f64, t28053: f64, t6888: f64, t7691: f64, t90739: f64, t1375: f64, t1386: f64, t20025: f64, t2016: f64, t26224: f64, t26225: f64, t26366: f64, t3887: f64, t5210: f64, t5354: f64, t539: f64, t56422: f64, t568: f64, t6460: f64, t6992: f64, t7722: f64, t81399: f64, t93906: f64, t97468: f64, t12020: f64, t1378: f64, t1390: f64, t16022: f64, t16030: f64, t16439: f64, t1807: f64, t1843: f64, t19648: f64, t1983: f64, t20022: f64, t20023: f64, t20029: f64, t20051: f64, t20060: f64, t2015: f64, t26226: f64, t26328: f64, t26348: f64, t26371: f64, t28111: f64, t28220: f64, t28224: f64, t3758: f64, t3882: f64, t5215: f64, t5321: f64, t5325: f64, t5326: f64, t533: f64, t56434: f64, t56580: f64, t56596: f64, t56607: f64, t6958: f64, t6963: f64, t7729: f64, t7749: f64, t7750: f64, t80711: f64, t81267: f64, t81282: f64, t81318: f64, t81375: f64, t90512: f64, t90515: f64, t90521: f64, t90585: f64, t90687: f64, t91441: f64, t91488: f64, t93335: f64, t93368: f64, t93387: f64, t93446: f64, t93899: f64, t96885: f64, t96893: f64, t96896: f64, t96900: f64, t96917: f64, t96920: f64, t96925: f64, t96929: f64, t96960: f64, t96999: f64, t97032: f64, t97075: f64, t97116: f64, t97154: f64, t97196: f64, t97496: f64, t97503: f64, t97519: f64, t97524: f64, t97527: f64, t97529: f64, t97552: f64, t97607: f64, t97611: f64, t97616: f64, t97619: f64, t97624: f64, t97626: f64, t97666: f64, t97717: f64, t97724: f64, t97729: f64, t97732: f64, t97740: f64, t24987: f64, t7756: f64, t2314: f64, t28025: f64, t4034: f64, t1266: f64, t28017: f64, t652: f64, t1845: f64, t5187: f64, t22574: f64, t8643: f64, t7688: f64, t1874: f64, t75560: f64, t19451: f64, t6525: f64, t25994: f64, t4028: f64, t55943: f64, t191: f64, t192: f64, t19537: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t97770 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2193(t28200, t6883, t225, t28053, t6888, t7691, t90739, t1375, t1386, t20025, t2016, t26224, t26225, t26366, t3887, t5210, t5354, t539, t56422, t568, t6460, t6992, t7722, t81399, t93906, t97468);
        let t97777 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2194(t12020, t1375, t1378, t1390, t16022, t16030, t16439, t1807, t1843, t19648, t1983, t20022, t20023, t20029, t20051, t20060, t2015, t2016, t26224, t26226, t26328, t26348, t26366, t26371, t28111, t28220, t28224, t3758, t3882, t3887, t5215, t5321, t5325, t5326, t533, t56434, t56580, t56596, t56607, t568, t6958, t6963, t7729, t7749, t7750, t80711, t81267, t81282, t81318, t81375, t90512, t90515, t90521, t90585, t90687, t91441, t91488, t93335, t93368, t93387, t93446, t93899, t96885, t96893, t96896, t96900, t96917, t96920, t96925, t96929, t96960, t96999, t97032, t97075, t97116, t97154, t97196, t97496, t97503, t97519, t97524, t97527, t97529, t97552, t97607, t97611, t97616, t97619, t97624, t97626, t97666, t97717, t97724, t97729, t97732, t97740, t97770);
        let (t97779, t97783, t97785, t97788, t97792) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2195(t24987, t7756, t2314, t28025, t4034, t1266, t28017, t652, t1845, t5187, t22574, t8643);
        let (t97794, t97796, t97798, t97800, t97802, t97804) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2196(t24987, t7688, t1874, t75560, t19451, t6525, t25994, t4028, t55943, t191, t192, t19537);
    (t97777, t97779, t97783, t97785, t97788, t97792, t97794, t97796, t97798, t97800, t97802, t97804)
}
