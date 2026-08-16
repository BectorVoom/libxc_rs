//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2243;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2244;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta623(t1462: f64, t152: f64, t9288: f64, t4211: f64, t9874: f64, t13119: f64, t2663: f64, t2517: f64, t4098: f64, t1472: f64, t9862: f64, t41274: f64, t13115: f64, t9932: f64, t32: f64, t4094: f64, t2659: f64, t1530: f64, t193: f64, t39658: f64, t46426: f64, t766: f64, t870: f64, t9458: f64, t13034: f64, t225: f64, t10104: f64, t10116: f64, t13029: f64, t13042: f64, t13050: f64, t13072: f64, t13460: f64, t13461: f64, t13463: f64, t1528: f64, t259: f64, t2597: f64, t2713: f64, t2718: f64, t2720: f64, t2743: f64, t40870: f64, t4147: f64, t4273: f64, t852: f64, t855: f64, t865: f64, t866: f64, t9590: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46432, t46434, t46436, t46438, t46439, t46444) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2243(t1462, t152, t9288, t4211, t9874, t13119, t2663, t2517, t4098, t1472, t9862, t41274);
        let (t46446, t46449, t46450) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2244(t13115, t9932, t32, t4094, t2659, t1530, t193, t39658, t46426, t46432, t46434, t46436, t46438, t46439, t46444, t766, t870, t9458);
        let t46481 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2245(t13034, t225, t10104, t10116, t13029, t13042, t13050, t13072, t13460, t13461, t13463, t1528, t259, t2597, t2713, t2718, t2720, t2743, t40870, t4147, t4273, t852, t855, t865, t866, t9590);
    (t46432, t46434, t46436, t46438, t46439, t46444, t46446, t46449, t46450, t46481)
}
