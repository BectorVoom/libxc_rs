//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta740 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2604;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2605;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta740<F: Float>(t11745: F, t15737: F, t1227: F, t13969: F, t15649: F, t43763: F, t44827: F, t11539: F, t1174: F, t14740: F, t14731: F, t135: F, t15666: F, t11665: F, t15572: F, t3515: F, t4983: F, t49850: F, t11818: F, t1213: F, t248: F, t5012: F, t11801: F, t5024: F, t11820: F, t5019: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t52908, t52917, t52919, t52926, t52932, t52935) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2604::<F>(t11745, t15737, t1227, t13969, t15649, t43763, t44827, t11539, t1174, t14740, t14731, t135, t15666);
        let (t52942, t52952, t52973, t52975, t52987) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2605::<F>(t11665, t15572, t3515, t4983, t49850, t11818, t1213, t248, t5012, t11801, t5024, t11820, t5019);
    (t52908, t52917, t52919, t52926, t52932, t52935, t52942, t52952, t52973, t52975, t52987)
}
