//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2243;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2244;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta623<F: Float>(t1462: F, t152: F, t9288: F, t4211: F, t9874: F, t13119: F, t2663: F, t2517: F, t4098: F, t1472: F, t9862: F, t41274: F, t13115: F, t9932: F, t32: F, t4094: F, t2659: F, t1530: F, t193: F, t39658: F, t46426: F, t766: F, t870: F, t9458: F, t13034: F, t225: F, t10104: F, t10116: F, t13029: F, t13042: F, t13050: F, t13072: F, t13460: F, t13461: F, t13463: F, t1528: F, t259: F, t2597: F, t2713: F, t2718: F, t2720: F, t2743: F, t40870: F, t4147: F, t4273: F, t852: F, t855: F, t865: F, t866: F, t9590: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46432, t46434, t46436, t46438, t46439, t46444) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2243::<F>(t1462, t152, t9288, t4211, t9874, t13119, t2663, t2517, t4098, t1472, t9862, t41274);
        let (t46446, t46449, t46450) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2244::<F>(t13115, t9932, t32, t4094, t2659, t1530, t193, t39658, t46426, t46432, t46434, t46436, t46438, t46439, t46444, t766, t870, t9458);
        let t46481 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2245::<F>(t13034, t225, t10104, t10116, t13029, t13042, t13050, t13072, t13460, t13461, t13463, t1528, t259, t2597, t2713, t2718, t2720, t2743, t40870, t4147, t4273, t852, t855, t865, t866, t9590);
    (t46432, t46434, t46436, t46438, t46439, t46444, t46446, t46449, t46450, t46481)
}
