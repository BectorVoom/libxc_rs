//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta672 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2013;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2014;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2015;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2016;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2017;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2018;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2019;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2020;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2021;
use chunk9::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2022;
use chunk10::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2023;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta672(t12021: f64, t1375: f64, t1843: f64, t20060: f64, t24082: f64, t29311: f64, t29372: f64, t3882: f64, t6439: f64, t6440: f64, t7199: f64, t7213: f64, t81264: f64, t90642: f64, t93338: f64, t93439: f64, t97513: f64, t97516: f64, t90807: f64, t90837: f64, t93473: f64, t93476: f64, t93483: f64, t93488: f64, t93489: f64, t93490: f64, t93491: f64, t93494: f64, t96935: f64, t96937: f64, t96941: f64, t96945: f64, t96949: f64, t96954: f64, t96958: f64, t1824: f64, t7918: f64, t1332: f64, t1352: f64, t19735: f64, t19805: f64, t2089: f64, t27074: f64, t29327: f64, t5250: f64, t5287: f64, t5334: f64, t5344: f64, t90868: f64, t90876: f64, t93524: f64, t93528: f64, t93529: f64, t93537: f64, t96962: f64, t96967: f64, t96972: f64, t96976: f64, t96979: f64, t2085: f64, t6414: f64, t19810: f64, t27078: f64, t81047: f64, t84480: f64, t90889: f64, t90900: f64, t90903: f64, t93562: f64, t93572: f64, t96986: f64, t96989: f64, t96993: f64, t96997: f64, t97002: f64, t97007: f64, t97014: f64, t97017: f64, t1814: f64, t27105: f64, t81076: f64, t84481: f64, t90925: f64, t97023: f64, t97026: f64, t97030: f64, t97036: f64, t97040: f64, t97043: f64, t97046: f64, t97049: f64, t97055: f64, t97059: f64, t97063: f64, t97067: f64, t97070: f64, t5230: f64, t7934: f64, t90980: f64, t93588: f64, t93589: f64, t93590: f64, t93592: f64, t93599: f64, t93600: f64, t97079: f64, t97083: f64, t97087: f64, t97091: f64, t97095: f64, t97106: f64, t97108: f64, t97111: f64, t97114: f64, t93633: f64, t93636: f64, t97202: f64, t97204: f64, t97206: f64, t97208: f64, t97210: f64, t97212: f64, t97214: f64, t97217: f64, t97219: f64, t97221: f64, t97223: f64, t97225: f64, t97227: f64, t97229: f64, t97231: f64, t93644: f64, t93645: f64, t93646: f64, t97236: f64, t97238: f64, t97240: f64, t97242: f64, t97244: f64, t97247: f64, t97249: f64, t97251: f64, t97253: f64, t97255: f64, t97257: f64, t97259: f64, t97261: f64, t97263: f64, t97266: f64, t91143: f64, t91149: f64, t91167: f64, t91179: f64, t93651: f64, t93652: f64, t93653: f64, t93657: f64, t97273: f64, t97277: f64, t97281: f64, t97283: f64, t97287: f64, t97291: f64, t97295: f64, t97299: f64, t97303: f64, t97307: f64, t80780: f64, t91206: f64, t91221: f64, t91223: f64, t93674: f64, t93682: f64, t97310: f64, t97315: f64, t97318: f64, t97320: f64, t97322: f64, t97326: f64, t97333: f64, t97337: f64, t97340: f64, t97342: f64, t97344: f64, t97347: f64, t80837: f64, t84514: f64, t84520: f64, t91244: f64, t91246: f64, t91247: f64, t93710: f64, t93711: f64, t93712: f64, t93715: f64, t93718: f64, t97352: f64, t97354: f64, t97359: f64, t97361: f64, t97363: f64, t97367: f64, t97372: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t102523 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2013(t12021, t1375, t1843, t20060, t24082, t29311, t29372, t3882, t6439, t6440, t7199, t7213, t81264, t90642, t93338, t93439, t97513, t97516);
        let t102558 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2014(t90807, t90837, t93473, t93476, t93483, t93488, t93489, t93490, t93491, t93494, t96935, t96937, t96941, t96945, t96949, t96954, t96958);
        let t102580 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2015(t1824, t7918, t1332, t1352, t19735, t19805, t2089, t27074, t29327, t5250, t5287, t5334, t5344, t90868, t90876, t93524, t93528, t93529, t93537, t96962, t96967, t96972, t96976, t96979);
        let (t102587, t102597) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2016(t2085, t6414, t1352, t19810, t27078, t5344, t81047, t84480, t90889, t90900, t90903, t93562, t93572, t96986, t96989, t96993, t96997, t97002, t97007, t97014, t97017);
        let t102614 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2017(t1814, t27105, t81076, t84481, t90925, t97023, t97026, t97030, t97036, t97040, t97043, t97046, t97049, t97055, t97059, t97063, t97067, t97070);
        let t102629 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2018(t5230, t7934, t90980, t93588, t93589, t93590, t93592, t93599, t93600, t97079, t97083, t97087, t97091, t97095, t97106, t97108, t97111, t97114);
        let t102647 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2019(t93633, t93636, t97202, t97204, t97206, t97208, t97210, t97212, t97214, t97217, t97219, t97221, t97223, t97225, t97227, t97229, t97231);
        let t102663 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2020(t93644, t93645, t93646, t97236, t97238, t97240, t97242, t97244, t97247, t97249, t97251, t97253, t97255, t97257, t97259, t97261, t97263, t97266);
        let t102679 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2021(t91143, t91149, t91167, t91179, t93651, t93652, t93653, t93657, t97273, t97277, t97281, t97283, t97287, t97291, t97295, t97299, t97303, t97307);
        let t102694 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2022(t80780, t91206, t91221, t91223, t93674, t93682, t97310, t97315, t97318, t97320, t97322, t97326, t97333, t97337, t97340, t97342, t97344, t97347);
        let t102705 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2023(t80837, t84514, t84520, t91244, t91246, t91247, t93710, t93711, t93712, t93715, t93718, t97352, t97354, t97359, t97361, t97363, t97367, t97372);
    (t102523, t102558, t102580, t102587, t102597, t102614, t102629, t102647, t102663, t102679, t102694, t102705)
}
