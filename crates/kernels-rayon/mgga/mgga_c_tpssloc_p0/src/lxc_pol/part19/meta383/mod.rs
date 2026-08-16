//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1431;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1432;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1433;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1434;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta383(t3368: f64, t3375: f64, t11292: f64, t1143: f64, t3324: f64, t3331: f64, t1124: f64, t11419: f64, t11282: f64, t43689: f64, t440: f64, t43713: f64, t43717: f64, t43721: f64, t43725: f64, t43727: f64, t43729: f64, t43734: f64, t43737: f64, t43740: f64, t43743: f64, t43746: f64, t43748: f64, t43750: f64, t43754: f64, t43776: f64, t43759: f64, t43766: f64, t43768: f64, t43770: f64, t43773: f64, t43833: f64, t43835: f64, t43837: f64, t43839: f64, t43842: f64, t43845: f64, t43848: f64, t43851: f64, t43855: f64, t43857: f64, t43859: f64, t43861: f64, t43863: f64, t43866: f64, t43869: f64, t43872: f64, t43875: f64, t43882: f64, t43884: f64, t43887: f64, t43890: f64, t43892: f64, t43819: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43794: f64, t43798: f64, t43802: f64, t43806: f64, t43811: f64, t43816: f64, t43823: f64, t43828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44202, t44205, t44211, t44214, t44220, t44223, t44243) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1431(t3368, t3375, t11292, t1143, t3324, t3331, t1124, t11419, t11282, t43689, t440, t43713, t43717, t43721, t43725, t43727, t43729, t43734, t43737, t43740, t43743, t43746, t43748, t43750, t43754);
        let t44258 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1432(t43776, t43759, t43766, t43768, t43770, t43773, t43833, t43835, t43837, t43839, t43842, t43845, t43848, t43851);
        let t44274 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1433(t43855, t43857, t43859, t43861, t43863, t43866, t43869, t43872, t43875, t43882, t43884, t43887, t43890, t43892);
        let t44289 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1434(t43819, t43780, t43782, t43784, t43786, t43788, t43794, t43798, t43802, t43806, t43811, t43816, t43823, t43828);
    (t44202, t44205, t44211, t44214, t44220, t44223, t44243, t44258, t44274, t44289)
}
