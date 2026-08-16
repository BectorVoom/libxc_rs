//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2402;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta623<F: Float>(t12189: F, t3745: F, t1314: F, t9580: F, t3741: F, t2566: F, t3732: F, t12204: F, t2229: F, t59: F, t60: F, t535: F, t9538: F) -> (F, F, F, F, F, F, F) {
        let (t40404, t40406, t40407, t40409, t40410, t40419, t40422) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2402::<F>(t12189, t3745, t1314, t9580, t3741, t2566, t3732, t12204, t2229, t59, t60, t535, t9538);
    (t40404, t40406, t40407, t40409, t40410, t40419, t40422)
}
