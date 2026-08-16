//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1423;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1424;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1425;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1426;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1427;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1428;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta381(t43776: f64, t43759: f64, t43766: f64, t43768: f64, t43770: f64, t43773: f64, t43833: f64, t43835: f64, t43837: f64, t43839: f64, t43842: f64, t43845: f64, t43848: f64, t43851: f64, t43855: f64, t43857: f64, t43859: f64, t43861: f64, t43863: f64, t43866: f64, t43869: f64, t43872: f64, t43875: f64, t43882: f64, t43884: f64, t43887: f64, t43890: f64, t43892: f64, t43819: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43794: f64, t43798: f64, t43802: f64, t43806: f64, t43811: f64, t43816: f64, t43823: f64, t43828: f64, t1099: f64, t1118: f64, t44021: f64, t3311: f64, t409: f64, t3314: f64, t43970: f64, t11185: f64, t11427: f64, t11190: f64, t3307: f64, t3264: f64, t3313: f64, t3315: f64, t11399: f64, t3403: f64, t11297: f64, t11303: f64, t11310: f64, t11361: f64, t11365: f64, t11430: f64, t11434: f64, t11437: f64, t1155: f64, t1157: f64, t3376: f64, t3377: f64, t3378: f64, t3395: f64, t3401: f64, t3404: f64, t43956: f64, t43958: f64, t43961: f64, t43963: f64, t43966: f64, t43973: f64, t43979: f64, t43984: f64, t43989: f64, t43994: f64, t11352: f64, t3351: f64, t11344: f64, t11350: f64, t1136: f64, t1138: f64, t11415: f64, t11420: f64, t11441: f64, t1148: f64, t1156: f64, t3332: f64, t3333: f64, t3334: f64, t3357: f64, t3359: f64, t3360: f64, t43911: f64, t43997: f64, t44000: f64, t44002: f64, t44006: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t44036 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1423(t43776, t43759, t43766, t43768, t43770, t43773, t43833, t43835, t43837, t43839, t43842, t43845, t43848, t43851);
        let t44052 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1424(t43855, t43857, t43859, t43861, t43863, t43866, t43869, t43872, t43875, t43882, t43884, t43887, t43890, t43892);
        let t44067 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1425(t43819, t43780, t43782, t43784, t43786, t43788, t43794, t43798, t43802, t43806, t43811, t43816, t43823, t43828);
        let (t44072, t44080, t44082) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1426(t1099, t1118, t44021, t44036, t44052, t44067, t3311, t409, t3314, t43970, t11185, t11427);
        let (t44085, t44089, t44092, t44115) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1427(t1118, t11190, t43970, t3307, t3264, t3313, t3315, t11399, t3403, t11297, t11303, t11310, t11361, t11365, t11430, t11434, t11437, t1155, t1157, t3376, t3377, t3378, t3395, t3401, t3404, t43956, t43958, t43961, t43963, t43966, t43973, t43979, t43984, t43989, t43994);
        let t44138 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1428(t11352, t3351, t11344, t11350, t1136, t1138, t11415, t11420, t11441, t1148, t1156, t3332, t3333, t3334, t3357, t3359, t3360, t43911, t43997, t44000, t44002, t44006, t44072, t44080, t44082, t44085, t44089, t44092);
    (t44072, t44080, t44082, t44085, t44089, t44092, t44115, t44138)
}
