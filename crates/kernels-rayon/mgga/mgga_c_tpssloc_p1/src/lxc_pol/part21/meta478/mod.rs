//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2070;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2071;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2072;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta478(t12832: f64, t16505: f64, t3: f64, t112: f64, t5363: f64, t111: f64, t1851: f64, t2319: f64, t576: f64, t4072: f64, t671: f64, t1458: f64, t2363: f64, t12521: f64, t12524: f64, t12813: f64, t1401: f64, t3938: f64, t3941: f64, t5371: f64, t5376: f64, t577: f64, t5392: f64, t9427: f64, t2433: f64, t5398: f64, t12603: f64, t12604: f64, t25: f64, t28: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16506, t16507, t16521, t16524, t16535, t16538, t16541) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2070(t12832, t16505, t3, t112, t5363, t111, t1851, t2319, t576, t4072, t671, t1458, t2363);
        let t16546 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2071(t12521, t12524, t12813, t1401, t1458, t16506, t16521, t16524, t16535, t16538, t16541, t2319, t2363, t3938, t3941, t4072, t5371, t5376, t577, t671);
        let (t16549, t16554, t16557) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2072(t5392, t9427, t2433, t5398, t12603, t12604);
        let t16558 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2073(t25, t28, t16557, zeta_threshold);
    (t16506, t16507, t16521, t16524, t16535, t16538, t16541, t16546, t16549, t16554, t16557, t16558)
}
