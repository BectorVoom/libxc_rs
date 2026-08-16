//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1858;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta469(t20371: f64, t20679: f64, t20692: f64, t20696: f64, t1458: f64, t6287: f64, t1774: f64, t5493: f64, t20347: f64, t510: f64, t16578: f64, t12861: f64, t40: f64, t52: f64, t20217: f64, t20234: f64, t4080: f64, t5398: f64, t73: f64, t9427: f64, t4087: f64, t76: f64, t9438: f64, t157: f64, t182: f64, t16587: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20698, t20702, t20717, t20720, t20723, t20724) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1858(t20371, t20679, t20692, t20696, t1458, t6287, t1774, t5493, t20347, t510, t16578, t12861);
        let (t20741, t20742, t20744, t20745) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1859(t40, t52, t20217, t20234, t4080, t5398, t73, t9427, t4087, t76, t9438, t157, t182, t16587, zeta_threshold);
    (t20698, t20702, t20717, t20720, t20723, t20724, t20741, t20742, t20744, t20745)
}
