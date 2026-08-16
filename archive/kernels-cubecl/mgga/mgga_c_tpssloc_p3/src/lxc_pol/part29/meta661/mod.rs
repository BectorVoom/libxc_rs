//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2195;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2196;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2197;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2198;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2199;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2200;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta661<F: Float>(t22633: F, t22635: F, t26354: F, t90506: F, t26211: F, t6883: F, t268: F, t557: F, t6559: F, t26333: F, t81326: F, t26338: F, t80650: F, t1985: F, t22934: F, t26193: F, t80722: F, t80725: F, t80728: F, t80738: F, t80744: F, t90598: F, t16413: F, t214: F, t225: F, t567: F, t26214: F, t26331: F, t3734: F, t22666: F, t26202: F, t22642: F, t22643: F, t7700: F, t22674: F, t6897: F, t22716: F, t7701: F, t1834: F, t212: F, t6890: F, t1373: F, t254: F, t81267: F, t12030: F, t12444: F, t1375: F, t22630: F, t26226: F, t26482: F, t3752: F, t3758: F, t3887: F, t3911: F, t5321: F, t568: F, t7722: F, t7729: F, t7749: F, t81264: F, t26215: F, t81228: F, t16436: F, t6889: F, t6906: F, t2015: F, t40590: F, t6907: F, t90544: F, t22662: F, t81284: F, t26203: F, t80645: F, t1385: F, t16022: F, t16474: F, t16475: F, t1843: F, t2016: F, t26224: F, t26348: F, t26371: F, t26471: F, t26477: F, t3912: F, t55069: F, t55134: F, t6958: F, t6993: F, t7750: F, t81282: F, t81319: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t90602, t90605, t90607, t90609, t90612) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2195::<F>(t22633, t22635, t26354, t90506, t26211, t6883, t268, t557, t6559, t26333, t81326, t26338, t80650);
        let t90621 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2196::<F>(t1985, t22934, t26193, t80722, t80725, t80728, t80738, t80744, t90598, t90602, t90605, t90609, t90612);
        let (t90626, t90634, t90639, t90642) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2197::<F>(t16413, t1985, t214, t225, t567, t22635, t26214, t26331, t3734, t22666, t26202, t22642, t22643, t7700);
        let (t90646, t90659, t90663, t90665, t90670) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2198::<F>(t22674, t26202, t6897, t22716, t7701, t1834, t212, t22642, t6890, t1373, t254, t81267);
        let t90677 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2199::<F>(t12030, t12444, t1375, t22630, t26226, t26482, t3752, t3758, t3887, t3911, t5321, t568, t7722, t7729, t7749, t81264, t90659, t90663, t90665, t90670);
        let (t90687, t90690, t90696, t90701) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2200::<F>(t26215, t81228, t81326, t16436, t1985, t6889, t6906, t2015, t40590, t6897, t6907, t90544);
        let t90725 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2201::<F>(t90701, t1985, t22662, t26193, t81284, t26203, t6883, t6897, t7700, t80645, t12030, t1375, t1385, t16022, t16436, t16474, t16475, t1843, t2015, t2016, t26224, t26348, t26371, t26471, t26477, t3758, t3887, t3912, t55069, t55134, t6958, t6993, t7750, t81282, t81319, t90687, t90690, t90696);
    (t90607, t90621, t90626, t90634, t90639, t90642, t90646, t90677, t90725)
}
