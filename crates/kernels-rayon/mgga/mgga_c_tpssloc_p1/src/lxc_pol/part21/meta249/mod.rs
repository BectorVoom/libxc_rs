//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta249 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1453;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1454;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1455;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1456;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1457;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1458;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1459;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta249(t248: f64, t3585: f64, t5971: f64, t1230: f64, t5979: f64, t5975: f64, t5985: f64, t5987: f64, t5991: f64, t6023: f64, t6026: f64, t6092: f64, t6094: f64, t6096: f64, t6100: f64, t6104: f64, t6108: f64, t475: f64, t1214: f64, t1734: f64, t3508: f64, t1213: f64, t1227: f64, t1737: f64, t1748: f64, t3506: f64, t3515: f64, t3542: f64, t3547: f64, t467: f64, t5005: f64, t5019: f64, t5024: f64, t5036: f64, t5041: f64, t6109: f64, t6197: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6203, t6207, t6211, t6218) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1453(t248, t3585, t5971, t1230, t5979, t5975, t5985, t5987, t5991, t6023, t6026, t6092, t6094, t6096, t6100, t6104, t6108);
        let t6219 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1454(t475, t6218);
        let (t6221, t6224) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1455(t1214, t248, t6219, t1734);
        let t6225 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1456(t3508, t6224);
        let (t6227, t6230) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1457(t1214, t248, t6225, t475, t6224);
        let (t6232, t6237) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1458(t1214, t248, t6230, t1213, t1227, t1737, t1748, t3506, t3515, t3542, t3547, t467, t5005, t5019, t5024, t5036, t5041, t6109, t6203, t6207, t6211, t6221, t6227);
        let t6238 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1459(t6197, t6237);
    (t6203, t6207, t6211, t6218, t6219, t6221, t6224, t6225, t6227, t6230, t6232, t6238)
}
