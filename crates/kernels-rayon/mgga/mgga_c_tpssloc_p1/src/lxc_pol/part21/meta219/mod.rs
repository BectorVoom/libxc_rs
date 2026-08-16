//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1334;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1335;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1336;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1337;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1338;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta219(t5250: f64, t5335: f64, t1825: f64, t3901: f64, t1380: f64, t5287: f64, t1338: f64, t68: f64, t544: f64, t1352: f64, t1834: f64, t5318: f64, t553: f64, t1332: f64, t1336: f64, t1381: f64, t1383: f64, t1814: f64, t1838: f64, t1840: f64, t3777: f64, t5230: f64, t5234: f64, t5334: f64, t564: f64, t1378: f64, t1375: f64, t1386: f64, t1843: f64, t3758: f64, t3882: f64, t5211: f64, t5213: f64, t5215: f64, t5217: f64, t5319: f64, t5321: f64, t5326: f64, t568: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5336, t5339, t5341, t5343, t5344) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1334(t5250, t5335, t1825, t3901, t1380, t5287, t1338, t68, t544);
        let (t5345, t5348) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1335(t1352, t5335, t1338, t1834);
        let (t5349, t5351, t5353) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1336(t1352, t5348, t5318, t553, t1332, t1336, t1381, t1383, t1814, t1838, t1840, t3777, t5230, t5234, t5334, t5336, t5339, t5341, t5344, t5345, t544, t564);
        let t5354 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1337(t1378, t5353);
        let t5356 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1338(t1375, t1386, t1843, t3758, t3882, t5211, t5213, t5215, t5217, t5319, t5321, t5326, t5354, t568);
    (t5336, t5339, t5341, t5343, t5344, t5345, t5348, t5349, t5351, t5353, t5354, t5356)
}
