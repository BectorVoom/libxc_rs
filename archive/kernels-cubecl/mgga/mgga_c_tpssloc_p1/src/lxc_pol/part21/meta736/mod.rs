//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta736 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2596;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2597;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta736<F: Float>(t13969: F, t15636: F, t3515: F, t1174: F, t44571: F, t4724: F, t11778: F, t43791: F, t1227: F, t49850: F, t4988: F, t15568: F, t3604: F, t11697: F, t15473: F, t3577: F, t11698: F, t15740: F, t10401: F, t15567: F, t3610: F, t11692: F, t15563: F, t15743: F, t3490: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t52586, t52599, t52601, t52609, t52615) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2596::<F>(t13969, t15636, t3515, t1174, t44571, t4724, t11778, t43791, t1227, t49850, t4988, t15568, t3604);
        let (t52619, t52621, t52627, t52628, t52649, t52653) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2597::<F>(t11697, t15473, t3577, t11698, t15740, t10401, t15567, t3610, t11692, t15563, t15743, t3490);
    (t52586, t52599, t52601, t52609, t52615, t52619, t52621, t52627, t52628, t52649, t52653)
}
