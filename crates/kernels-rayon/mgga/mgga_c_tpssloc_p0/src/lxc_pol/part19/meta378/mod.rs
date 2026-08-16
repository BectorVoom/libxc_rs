//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1411;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1412;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1413;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1414;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1415;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta378(t11265: f64, t3271: f64, t3279: f64, t11243: f64, t39267: f64, t404: f64, t410: f64, t1100: f64, t43832: f64, t3270: f64, t407: f64, t3287: f64, t43855: f64, t43857: f64, t43859: f64, t43861: f64, t43863: f64, t43866: f64, t43869: f64, t43819: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43794: f64, t43798: f64, t43802: f64, t43806: f64, t43811: f64, t43816: f64, t43823: f64, t43828: f64, t43756: f64, t43853: f64, t1147: f64, t1156: f64, t1164: f64, t11940: f64, t11947: f64, t1254: f64, t193: f64, t336: f64, t3633: f64, t3637: f64, t3640: f64, t43670: f64, t43672: f64, t43674: f64, t43678: f64, t43683: f64, t43685: f64, t43687: f64, t43695: f64, t43702: f64, t43703: f64, t43706: f64, t4700: f64, t11292: f64, t43679: f64, t43748: f64, t43750: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43872, t43875, t43882, t43884, t43887, t43890, t43892) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1411(t11265, t3271, t3279, t11243, t39267, t404, t410, t1100, t43832, t3270, t407, t3287);
        let t43894 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1412(t43855, t43857, t43859, t43861, t43863, t43866, t43869, t43872, t43875, t43882, t43884, t43887, t43890, t43892);
        let t43909 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1413(t43819, t43780, t43782, t43784, t43786, t43788, t43794, t43798, t43802, t43806, t43811, t43816, t43823, t43828);
        let (t43911, t43915, t43920) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1414(t43756, t43853, t43894, t43909, t1147, t1156, t1164, t11940, t11947, t1254, t193, t336, t3633, t3637, t3640, t43670, t43672, t43674, t43678, t43683, t43685, t43687, t43695, t43702, t43703, t43706, t4700);
        let (t43924, t43936) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1415(t11292, t1156, t1164, t43679, t43748, t43750, t43780, t43782, t43784, t43786, t43788, t43794, t43798, t43802, t43806);
    (t43872, t43875, t43882, t43884, t43887, t43890, t43892, t43911, t43915, t43920, t43924, t43936)
}
