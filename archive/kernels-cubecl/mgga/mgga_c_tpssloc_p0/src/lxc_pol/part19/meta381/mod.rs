//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1423;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1424;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1425;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1426;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1427;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1428;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta381<F: Float>(t43776: F, t43759: F, t43766: F, t43768: F, t43770: F, t43773: F, t43833: F, t43835: F, t43837: F, t43839: F, t43842: F, t43845: F, t43848: F, t43851: F, t43855: F, t43857: F, t43859: F, t43861: F, t43863: F, t43866: F, t43869: F, t43872: F, t43875: F, t43882: F, t43884: F, t43887: F, t43890: F, t43892: F, t43819: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F, t43811: F, t43816: F, t43823: F, t43828: F, t1099: F, t1118: F, t44021: F, t3311: F, t409: F, t3314: F, t43970: F, t11185: F, t11427: F, t11190: F, t3307: F, t3264: F, t3313: F, t3315: F, t11399: F, t3403: F, t11297: F, t11303: F, t11310: F, t11361: F, t11365: F, t11430: F, t11434: F, t11437: F, t1155: F, t1157: F, t3376: F, t3377: F, t3378: F, t3395: F, t3401: F, t3404: F, t43956: F, t43958: F, t43961: F, t43963: F, t43966: F, t43973: F, t43979: F, t43984: F, t43989: F, t43994: F, t11352: F, t3351: F, t11344: F, t11350: F, t1136: F, t1138: F, t11415: F, t11420: F, t11441: F, t1148: F, t1156: F, t3332: F, t3333: F, t3334: F, t3357: F, t3359: F, t3360: F, t43911: F, t43997: F, t44000: F, t44002: F, t44006: F) -> (F, F, F, F, F, F, F, F) {
        let t44036 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1423::<F>(t43776, t43759, t43766, t43768, t43770, t43773, t43833, t43835, t43837, t43839, t43842, t43845, t43848, t43851);
        let t44052 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1424::<F>(t43855, t43857, t43859, t43861, t43863, t43866, t43869, t43872, t43875, t43882, t43884, t43887, t43890, t43892);
        let t44067 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1425::<F>(t43819, t43780, t43782, t43784, t43786, t43788, t43794, t43798, t43802, t43806, t43811, t43816, t43823, t43828);
        let (t44072, t44080, t44082) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1426::<F>(t1099, t1118, t44021, t44036, t44052, t44067, t3311, t409, t3314, t43970, t11185, t11427);
        let (t44085, t44089, t44092, t44115) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1427::<F>(t1118, t11190, t43970, t3307, t3264, t3313, t3315, t11399, t3403, t11297, t11303, t11310, t11361, t11365, t11430, t11434, t11437, t1155, t1157, t3376, t3377, t3378, t3395, t3401, t3404, t43956, t43958, t43961, t43963, t43966, t43973, t43979, t43984, t43989, t43994);
        let t44138 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1428::<F>(t11352, t3351, t11344, t11350, t1136, t1138, t11415, t11420, t11441, t1148, t1156, t3332, t3333, t3334, t3357, t3359, t3360, t43911, t43997, t44000, t44002, t44006, t44072, t44080, t44082, t44085, t44089, t44092);
    (t44072, t44080, t44082, t44085, t44089, t44092, t44115, t44138)
}
