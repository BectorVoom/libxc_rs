//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1431;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1432;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1433;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1434;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta383<F: Float>(t3368: F, t3375: F, t11292: F, t1143: F, t3324: F, t3331: F, t1124: F, t11419: F, t11282: F, t43689: F, t440: F, t43713: F, t43717: F, t43721: F, t43725: F, t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43748: F, t43750: F, t43754: F, t43776: F, t43759: F, t43766: F, t43768: F, t43770: F, t43773: F, t43833: F, t43835: F, t43837: F, t43839: F, t43842: F, t43845: F, t43848: F, t43851: F, t43855: F, t43857: F, t43859: F, t43861: F, t43863: F, t43866: F, t43869: F, t43872: F, t43875: F, t43882: F, t43884: F, t43887: F, t43890: F, t43892: F, t43819: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F, t43811: F, t43816: F, t43823: F, t43828: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44202, t44205, t44211, t44214, t44220, t44223, t44243) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1431::<F>(t3368, t3375, t11292, t1143, t3324, t3331, t1124, t11419, t11282, t43689, t440, t43713, t43717, t43721, t43725, t43727, t43729, t43734, t43737, t43740, t43743, t43746, t43748, t43750, t43754);
        let t44258 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1432::<F>(t43776, t43759, t43766, t43768, t43770, t43773, t43833, t43835, t43837, t43839, t43842, t43845, t43848, t43851);
        let t44274 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1433::<F>(t43855, t43857, t43859, t43861, t43863, t43866, t43869, t43872, t43875, t43882, t43884, t43887, t43890, t43892);
        let t44289 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1434::<F>(t43819, t43780, t43782, t43784, t43786, t43788, t43794, t43798, t43802, t43806, t43811, t43816, t43823, t43828);
    (t44202, t44205, t44211, t44214, t44220, t44223, t44243, t44258, t44274, t44289)
}
