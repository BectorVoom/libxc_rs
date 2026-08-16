//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta178 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1132;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1133;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1134;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1135;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta178<F: Float>(t1527: F, t865: F, t2718: F, t2627: F, t68: F, t226: F, t1509: F, t252: F, t4182: F, t1510: F, t2732: F, t4234: F, t860: F, t814: F, t829: F, t1519: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t4273 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1132::<F>(t1527, t865, t2718);
        let (t4280, t4281) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1133::<F>(t2627, t68, t226);
        let t4282 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1134::<F>(t1509, t252);
        let (t4283, t4286, t4288, t4290, t4291) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1135::<F>(t4182, t4282, t1510, t2732, t4234, t860, t68, t814, t226);
        let (t4292, t4295) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1136::<F>(t4282, t829, t1519, t814);
    (t4273, t4280, t4281, t4282, t4283, t4286, t4288, t4290, t4291, t4292, t4295)
}
