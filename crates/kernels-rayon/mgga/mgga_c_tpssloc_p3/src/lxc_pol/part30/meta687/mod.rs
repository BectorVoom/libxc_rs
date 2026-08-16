//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta687 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2177;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2178;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2179;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2180;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2181;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2182;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2183;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta687(t225: f64, t28108: f64, t22674: f64, t28232: f64, t6897: f64, t28195: f64, t6883: f64, t22633: f64, t22635: f64, t26337: f64, t5353: f64, t5325: f64, t90488: f64, t1307: f64, t567: f64, t6330: f64, t90591: f64, t28199: f64, t794: f64, t1985: f64, t20009: f64, t214: f64, t1375: f64, t1386: f64, t16460: f64, t19647: f64, t20044: f64, t20050: f64, t20060: f64, t22670: f64, t26224: f64, t26225: f64, t26371: f64, t26472: f64, t26482: f64, t3887: f64, t5215: f64, t5321: f64, t6461: f64, t6963: f64, t6993: f64, t7749: f64, t7750: f64, t81311: f64, t90696: f64, t90724: f64, t3886: f64, t6439: f64, t26193: f64, t26202: f64, t6888: f64, t6891: f64, t97511: f64, t28116: f64, t80650: f64, t1808: f64, t254: f64, t1377: f64, t6347: f64, t1385: f64, t1842: f64, t90516: f64, t1992: f64, t26355: f64, t90566: f64, t26331: f64, t20022: f64, t6889: f64, t6906: f64, t28192: f64, t80727: f64, t20029: f64, t26471: f64, t91487: f64, t6460: f64, t16030: f64, t1843: f64, t22656: f64, t26348: f64, t26477: f64, t28111: f64, t28186: f64, t28220: f64, t3758: f64, t3882: f64, t5326: f64, t6440: f64, t7729: f64, t90732: f64, t91491: f64, t26189: f64, t22892: f64, t7691: f64, t90544: f64, t1835: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97558, t97571, t97573, t97577, t97583) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2177(t225, t28108, t22674, t28232, t6897, t28195, t6883, t22633, t22635, t26337, t5353, t5325, t90488);
        let (t97588, t97599, t97604) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2178(t1307, t22635, t567, t6330, t90591, t28199, t6897, t794, t1985, t20009, t214, t225);
        let t97607 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2179(t1375, t1386, t16460, t19647, t20044, t20050, t20060, t22670, t26224, t26225, t26371, t26472, t26482, t3887, t5215, t5321, t5353, t6461, t6963, t6993, t7749, t7750, t81311, t90696, t90724, t97558, t97571, t97573, t97577, t97583, t97588, t97599, t97604);
        let (t97611, t97616, t97619, t97624) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2180(t3886, t6439, t1307, t22633, t22635, t1985, t26193, t26202, t6888, t6891, t97511, t28116, t80650);
        let (t97626, t97640, t97644, t97647) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2181(t1808, t254, t1377, t6347, t1385, t22633, t22635, t1842, t90516, t1992, t26355, t90566);
        let t97666 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2182(t1307, t22635, t26331, t567, t6347, t1985, t20022, t6889, t6906, t28192, t80727, t1375, t1842, t20029, t26471, t26472, t26482, t3887, t5215, t5321, t6993, t91487, t97640, t97644, t97647);
        let t97717 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2183(t1307, t1377, t22633, t22635, t6460, t1375, t1385, t16030, t1843, t22656, t22670, t26348, t26477, t28111, t28186, t28220, t3758, t3882, t3887, t5321, t5326, t6440, t7729, t90732, t91491);
        let (t97724, t97729, t97732, t97740) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2184(t1307, t1842, t22635, t26331, t26337, t26189, t26193, t6888, t22892, t7691, t90544, t1835, t254);
    (t97607, t97611, t97616, t97619, t97624, t97626, t97666, t97717, t97724, t97729, t97732, t97740)
}
