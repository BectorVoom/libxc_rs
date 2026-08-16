//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta656 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2290;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2291;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2292;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2293;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2294;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2295;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2296;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta656(t22633: f64, t22635: f64, t26354: f64, t90506: f64, t26211: f64, t6883: f64, t268: f64, t557: f64, t6559: f64, t26333: f64, t81326: f64, t26338: f64, t80650: f64, t1985: f64, t22934: f64, t26193: f64, t80722: f64, t80725: f64, t80728: f64, t80738: f64, t80744: f64, t90598: f64, t16413: f64, t214: f64, t225: f64, t567: f64, t26214: f64, t26331: f64, t3734: f64, t22666: f64, t26202: f64, t22642: f64, t22643: f64, t7700: f64, t22674: f64, t6897: f64, t22716: f64, t7701: f64, t1834: f64, t212: f64, t6890: f64, t1373: f64, t254: f64, t81267: f64, t12030: f64, t12444: f64, t1375: f64, t22630: f64, t26226: f64, t26482: f64, t3752: f64, t3758: f64, t3887: f64, t3911: f64, t5321: f64, t568: f64, t7722: f64, t7729: f64, t7749: f64, t81264: f64, t26215: f64, t81228: f64, t16436: f64, t6889: f64, t6906: f64, t2015: f64, t40590: f64, t6907: f64, t90544: f64, t22662: f64, t81284: f64, t26203: f64, t80645: f64, t1385: f64, t16022: f64, t16474: f64, t16475: f64, t1843: f64, t2016: f64, t26224: f64, t26348: f64, t26371: f64, t26471: f64, t26477: f64, t3912: f64, t55069: f64, t55134: f64, t6958: f64, t6993: f64, t7750: f64, t81282: f64, t81319: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90602, t90605, t90607, t90609, t90612) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2290(t22633, t22635, t26354, t90506, t26211, t6883, t268, t557, t6559, t26333, t81326, t26338, t80650);
        let t90621 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2291(t1985, t22934, t26193, t80722, t80725, t80728, t80738, t80744, t90598, t90602, t90605, t90609, t90612);
        let (t90626, t90634, t90639, t90642) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2292(t16413, t1985, t214, t225, t567, t22635, t26214, t26331, t3734, t22666, t26202, t22642, t22643, t7700);
        let (t90646, t90659, t90663, t90665, t90670) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2293(t22674, t26202, t6897, t22716, t7701, t1834, t212, t22642, t6890, t1373, t254, t81267);
        let t90677 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2294(t12030, t12444, t1375, t22630, t26226, t26482, t3752, t3758, t3887, t3911, t5321, t568, t7722, t7729, t7749, t81264, t90659, t90663, t90665, t90670);
        let (t90687, t90690, t90696, t90701) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2295(t26215, t81228, t81326, t16436, t1985, t6889, t6906, t2015, t40590, t6897, t6907, t90544);
        let t90725 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2296(t90701, t1985, t22662, t26193, t81284, t26203, t6883, t6897, t7700, t80645, t12030, t1375, t1385, t16022, t16436, t16474, t16475, t1843, t2015, t2016, t26224, t26348, t26371, t26471, t26477, t3758, t3887, t3912, t55069, t55134, t6958, t6993, t7750, t81282, t81319, t90687, t90690, t90696);
    (t90607, t90621, t90626, t90634, t90639, t90642, t90646, t90677, t90725)
}
