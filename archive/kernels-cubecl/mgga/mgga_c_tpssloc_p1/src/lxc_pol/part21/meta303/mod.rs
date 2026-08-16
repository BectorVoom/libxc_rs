//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1646;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1647;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1648;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta303<F: Float>(t11135: F, t11203: F, t1127: F, t3355: F, t427: F, t3358: F, t435: F, t1147: F, t3368: F, t1143: F, t3400: F, t11292: F, t440: F) -> (F, F, F, F, F, F, F, F) {
        let (t11314, t11317, t11349, t11350) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1646::<F>(t11135, t11203, t1127, t3355, t427);
        let (t11352, t11356, t11361) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1647::<F>(t3358, t435, t1147, t3368, t1143, t3400);
        let t11365 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1648::<F>(t11292, t440);
    (t11314, t11317, t11349, t11350, t11352, t11356, t11361, t11365)
}
