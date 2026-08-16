//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1411;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1412;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1413;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1414;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1415;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta378<F: Float>(t11265: F, t3271: F, t3279: F, t11243: F, t39267: F, t404: F, t410: F, t1100: F, t43832: F, t3270: F, t407: F, t3287: F, t43855: F, t43857: F, t43859: F, t43861: F, t43863: F, t43866: F, t43869: F, t43819: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F, t43811: F, t43816: F, t43823: F, t43828: F, t43756: F, t43853: F, t1147: F, t1156: F, t1164: F, t11940: F, t11947: F, t1254: F, t193: F, t336: F, t3633: F, t3637: F, t3640: F, t43670: F, t43672: F, t43674: F, t43678: F, t43683: F, t43685: F, t43687: F, t43695: F, t43702: F, t43703: F, t43706: F, t4700: F, t11292: F, t43679: F, t43748: F, t43750: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t43872, t43875, t43882, t43884, t43887, t43890, t43892) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1411::<F>(t11265, t3271, t3279, t11243, t39267, t404, t410, t1100, t43832, t3270, t407, t3287);
        let t43894 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1412::<F>(t43855, t43857, t43859, t43861, t43863, t43866, t43869, t43872, t43875, t43882, t43884, t43887, t43890, t43892);
        let t43909 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1413::<F>(t43819, t43780, t43782, t43784, t43786, t43788, t43794, t43798, t43802, t43806, t43811, t43816, t43823, t43828);
        let (t43911, t43915, t43920) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1414::<F>(t43756, t43853, t43894, t43909, t1147, t1156, t1164, t11940, t11947, t1254, t193, t336, t3633, t3637, t3640, t43670, t43672, t43674, t43678, t43683, t43685, t43687, t43695, t43702, t43703, t43706, t4700);
        let (t43924, t43936) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1415::<F>(t11292, t1156, t1164, t43679, t43748, t43750, t43780, t43782, t43784, t43786, t43788, t43794, t43798, t43802, t43806);
    (t43872, t43875, t43882, t43884, t43887, t43890, t43892, t43911, t43915, t43920, t43924, t43936)
}
