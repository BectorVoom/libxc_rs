//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1858;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta469<F: Float>(t20371: F, t20679: F, t20692: F, t20696: F, t1458: F, t6287: F, t1774: F, t5493: F, t20347: F, t510: F, t16578: F, t12861: F, t40: F, t52: F, t20217: F, t20234: F, t4080: F, t5398: F, t73: F, t9427: F, t4087: F, t76: F, t9438: F, t157: F, t182: F, t16587: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t20698, t20702, t20717, t20720, t20723, t20724) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1858::<F>(t20371, t20679, t20692, t20696, t1458, t6287, t1774, t5493, t20347, t510, t16578, t12861);
        let (t20741, t20742, t20744, t20745) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1859::<F>(t40, t52, t20217, t20234, t4080, t5398, t73, t9427, t4087, t76, t9438, t157, t182, t16587, zeta_threshold);
    (t20698, t20702, t20717, t20720, t20723, t20724, t20741, t20742, t20744, t20745)
}
