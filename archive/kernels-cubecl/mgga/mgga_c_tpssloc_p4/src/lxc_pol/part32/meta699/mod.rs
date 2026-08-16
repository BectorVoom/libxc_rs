//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta699 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2185;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2186;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2187;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2188;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2189;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2190;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2191;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2192;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta699<F: Float>(t225: F, t28108: F, t22674: F, t28232: F, t6897: F, t28195: F, t6883: F, t22633: F, t22635: F, t26337: F, t5353: F, t5325: F, t90488: F, t1307: F, t567: F, t6330: F, t90591: F, t28199: F, t794: F, t1985: F, t20009: F, t214: F, t1375: F, t1386: F, t16460: F, t19647: F, t20044: F, t20050: F, t20060: F, t22670: F, t26224: F, t26225: F, t26371: F, t26472: F, t26482: F, t3887: F, t5215: F, t5321: F, t6461: F, t6963: F, t6993: F, t7749: F, t7750: F, t81311: F, t90696: F, t90724: F, t3886: F, t6439: F, t26193: F, t26202: F, t6888: F, t6891: F, t97511: F, t28116: F, t80650: F, t1808: F, t254: F, t1377: F, t6347: F, t1385: F, t1842: F, t90516: F, t1992: F, t26355: F, t90566: F, t26331: F, t20022: F, t6889: F, t6906: F, t28192: F, t80727: F, t20029: F, t26471: F, t91487: F, t6460: F, t16030: F, t1843: F, t22656: F, t26348: F, t26477: F, t28111: F, t28186: F, t28220: F, t3758: F, t3882: F, t5326: F, t6440: F, t7729: F, t90732: F, t91491: F, t26189: F, t22892: F, t7691: F, t90544: F, t1835: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t97558, t97571, t97573, t97577, t97583) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2185::<F>(t225, t28108, t22674, t28232, t6897, t28195, t6883, t22633, t22635, t26337, t5353, t5325, t90488);
        let (t97588, t97599, t97604) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2186::<F>(t1307, t22635, t567, t6330, t90591, t28199, t6897, t794, t1985, t20009, t214, t225);
        let t97607 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2187::<F>(t1375, t1386, t16460, t19647, t20044, t20050, t20060, t22670, t26224, t26225, t26371, t26472, t26482, t3887, t5215, t5321, t5353, t6461, t6963, t6993, t7749, t7750, t81311, t90696, t90724, t97558, t97571, t97573, t97577, t97583, t97588, t97599, t97604);
        let (t97611, t97616, t97619, t97624) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2188::<F>(t3886, t6439, t1307, t22633, t22635, t1985, t26193, t26202, t6888, t6891, t97511, t28116, t80650);
        let (t97626, t97640, t97644, t97647) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2189::<F>(t1808, t254, t1377, t6347, t1385, t22633, t22635, t1842, t90516, t1992, t26355, t90566);
        let t97666 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2190::<F>(t1307, t22635, t26331, t567, t6347, t1985, t20022, t6889, t6906, t28192, t80727, t1375, t1842, t20029, t26471, t26472, t26482, t3887, t5215, t5321, t6993, t91487, t97640, t97644, t97647);
        let t97717 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2191::<F>(t1307, t1377, t22633, t22635, t6460, t1375, t1385, t16030, t1843, t22656, t22670, t26348, t26477, t28111, t28186, t28220, t3758, t3882, t3887, t5321, t5326, t6440, t7729, t90732, t91491);
        let (t97724, t97729, t97732, t97740) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2192::<F>(t1307, t1842, t22635, t26331, t26337, t26189, t26193, t6888, t22892, t7691, t90544, t1835, t254);
    (t97607, t97611, t97616, t97619, t97624, t97626, t97666, t97717, t97724, t97729, t97732, t97740)
}
