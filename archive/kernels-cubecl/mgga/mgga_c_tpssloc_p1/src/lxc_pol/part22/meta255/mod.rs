//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta255 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1377;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1378;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1379;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1380;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta255<F: Float>(t3402: F, t448: F, t3399: F, t445: F, t1143: F, t3375: F, t1124: F, t3331: F, t11282: F, t440: F, t11135: F, t11203: F, t1127: F, t3355: F, t427: F, t3358: F, t435: F, t3400: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t11285 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1377::<F>(t3402, t448);
        let t11292 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1378::<F>(t3399, t445);
        let (t11297, t11303, t11310) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1379::<F>(t1143, t3375, t1124, t3331, t11282, t440);
        let (t11314, t11317, t11349, t11350, t11352, t11361, t11365) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1380::<F>(t11135, t11203, t1127, t3355, t427, t3358, t435, t1143, t3400, t11292, t440);
    (t11285, t11292, t11297, t11303, t11310, t11314, t11317, t11349, t11350, t11352, t11361, t11365)
}
