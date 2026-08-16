//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2070;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2071;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2072;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta478<F: Float>(t12832: F, t16505: F, t3: F, t112: F, t5363: F, t111: F, t1851: F, t2319: F, t576: F, t4072: F, t671: F, t1458: F, t2363: F, t12521: F, t12524: F, t12813: F, t1401: F, t3938: F, t3941: F, t5371: F, t5376: F, t577: F, t5392: F, t9427: F, t2433: F, t5398: F, t12603: F, t12604: F, t25: F, t28: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16506, t16507, t16521, t16524, t16535, t16538, t16541) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2070::<F>(t12832, t16505, t3, t112, t5363, t111, t1851, t2319, t576, t4072, t671, t1458, t2363);
        let t16546 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2071::<F>(t12521, t12524, t12813, t1401, t1458, t16506, t16521, t16524, t16535, t16538, t16541, t2319, t2363, t3938, t3941, t4072, t5371, t5376, t577, t671);
        let (t16549, t16554, t16557) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2072::<F>(t5392, t9427, t2433, t5398, t12603, t12604);
        let t16558 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2073::<F>(t25, t28, t16557, zeta_threshold);
    (t16506, t16507, t16521, t16524, t16535, t16538, t16541, t16546, t16549, t16554, t16557, t16558)
}
