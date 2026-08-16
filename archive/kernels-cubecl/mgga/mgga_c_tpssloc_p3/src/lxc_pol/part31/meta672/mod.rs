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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta672<F: Float>(t12021: F, t1375: F, t1843: F, t20060: F, t24082: F, t29311: F, t29372: F, t3882: F, t6439: F, t6440: F, t7199: F, t7213: F, t81264: F, t90642: F, t93338: F, t93439: F, t97513: F, t97516: F, t90807: F, t90837: F, t93473: F, t93476: F, t93483: F, t93488: F, t93489: F, t93490: F, t93491: F, t93494: F, t96935: F, t96937: F, t96941: F, t96945: F, t96949: F, t96954: F, t96958: F, t1824: F, t7918: F, t1332: F, t1352: F, t19735: F, t19805: F, t2089: F, t27074: F, t29327: F, t5250: F, t5287: F, t5334: F, t5344: F, t90868: F, t90876: F, t93524: F, t93528: F, t93529: F, t93537: F, t96962: F, t96967: F, t96972: F, t96976: F, t96979: F, t2085: F, t6414: F, t19810: F, t27078: F, t81047: F, t84480: F, t90889: F, t90900: F, t90903: F, t93562: F, t93572: F, t96986: F, t96989: F, t96993: F, t96997: F, t97002: F, t97007: F, t97014: F, t97017: F, t1814: F, t27105: F, t81076: F, t84481: F, t90925: F, t97023: F, t97026: F, t97030: F, t97036: F, t97040: F, t97043: F, t97046: F, t97049: F, t97055: F, t97059: F, t97063: F, t97067: F, t97070: F, t5230: F, t7934: F, t90980: F, t93588: F, t93589: F, t93590: F, t93592: F, t93599: F, t93600: F, t97079: F, t97083: F, t97087: F, t97091: F, t97095: F, t97106: F, t97108: F, t97111: F, t97114: F, t93633: F, t93636: F, t97202: F, t97204: F, t97206: F, t97208: F, t97210: F, t97212: F, t97214: F, t97217: F, t97219: F, t97221: F, t97223: F, t97225: F, t97227: F, t97229: F, t97231: F, t93644: F, t93645: F, t93646: F, t97236: F, t97238: F, t97240: F, t97242: F, t97244: F, t97247: F, t97249: F, t97251: F, t97253: F, t97255: F, t97257: F, t97259: F, t97261: F, t97263: F, t97266: F, t91143: F, t91149: F, t91167: F, t91179: F, t93651: F, t93652: F, t93653: F, t93657: F, t97273: F, t97277: F, t97281: F, t97283: F, t97287: F, t97291: F, t97295: F, t97299: F, t97303: F, t97307: F, t80780: F, t91206: F, t91221: F, t91223: F, t93674: F, t93682: F, t97310: F, t97315: F, t97318: F, t97320: F, t97322: F, t97326: F, t97333: F, t97337: F, t97340: F, t97342: F, t97344: F, t97347: F, t80837: F, t84514: F, t84520: F, t91244: F, t91246: F, t91247: F, t93710: F, t93711: F, t93712: F, t93715: F, t93718: F, t97352: F, t97354: F, t97359: F, t97361: F, t97363: F, t97367: F, t97372: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t102523 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2013::<F>(t12021, t1375, t1843, t20060, t24082, t29311, t29372, t3882, t6439, t6440, t7199, t7213, t81264, t90642, t93338, t93439, t97513, t97516);
        let t102558 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2014::<F>(t90807, t90837, t93473, t93476, t93483, t93488, t93489, t93490, t93491, t93494, t96935, t96937, t96941, t96945, t96949, t96954, t96958);
        let t102580 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2015::<F>(t1824, t7918, t1332, t1352, t19735, t19805, t2089, t27074, t29327, t5250, t5287, t5334, t5344, t90868, t90876, t93524, t93528, t93529, t93537, t96962, t96967, t96972, t96976, t96979);
        let (t102587, t102597) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2016::<F>(t2085, t6414, t1352, t19810, t27078, t5344, t81047, t84480, t90889, t90900, t90903, t93562, t93572, t96986, t96989, t96993, t96997, t97002, t97007, t97014, t97017);
        let t102614 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2017::<F>(t1814, t27105, t81076, t84481, t90925, t97023, t97026, t97030, t97036, t97040, t97043, t97046, t97049, t97055, t97059, t97063, t97067, t97070);
        let t102629 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2018::<F>(t5230, t7934, t90980, t93588, t93589, t93590, t93592, t93599, t93600, t97079, t97083, t97087, t97091, t97095, t97106, t97108, t97111, t97114);
        let t102647 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2019::<F>(t93633, t93636, t97202, t97204, t97206, t97208, t97210, t97212, t97214, t97217, t97219, t97221, t97223, t97225, t97227, t97229, t97231);
        let t102663 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2020::<F>(t93644, t93645, t93646, t97236, t97238, t97240, t97242, t97244, t97247, t97249, t97251, t97253, t97255, t97257, t97259, t97261, t97263, t97266);
        let t102679 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2021::<F>(t91143, t91149, t91167, t91179, t93651, t93652, t93653, t93657, t97273, t97277, t97281, t97283, t97287, t97291, t97295, t97299, t97303, t97307);
        let t102694 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2022::<F>(t80780, t91206, t91221, t91223, t93674, t93682, t97310, t97315, t97318, t97320, t97322, t97326, t97333, t97337, t97340, t97342, t97344, t97347);
        let t102705 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2023::<F>(t80837, t84514, t84520, t91244, t91246, t91247, t93710, t93711, t93712, t93715, t93718, t97352, t97354, t97359, t97361, t97363, t97367, t97372);
    (t102523, t102558, t102580, t102587, t102597, t102614, t102629, t102647, t102663, t102679, t102694, t102705)
}
