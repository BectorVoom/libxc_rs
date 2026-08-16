//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta188 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1178;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1179;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1180;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta188(t1597: f64, t984: f64, t343: f64, t4546: f64, t1593: f64, t1600: f64, t2958: f64, t2960: f64, t2969: f64, t2972: f64, t2975: f64, t2986: f64, t4507: f64, t4511: f64, t4515: f64, t4519: f64, t4523: f64, t4529: f64, t4532: f64, t4543: f64, t973: f64, t381: f64, t1049: f64, t1603: f64, t1604: f64, t225: f64, t1625: f64, t990: f64, t4343: f64, t977: f64, t2979: f64, t4338: f64, t1539: f64, t248: f64, t3051: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4548, t4549, t4552) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1178(t1597, t984, t343, t4546, t1593, t1600, t2958, t2960, t2969, t2972, t2975, t2986, t4507, t4511, t4515, t4519, t4523, t4529, t4532, t4543, t973);
        let (t4553, t4555, t4557) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1179(t381, t4552, t1049, t1603, t1604, t225);
        let (t4559, t4562, t4565, t4571) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1180(t1625, t990, t4343, t977, t2979, t4338, t1539, t248, t3051);
    (t4548, t4549, t4552, t4553, t4555, t4557, t4559, t4562, t4565, t4571)
}
