//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta370 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1815;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1816;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta370<F: Float>(t13615: F, t901: F, t2815: F, t4370: F, t896: F, t2807: F, t4378: F, t2798: F, t4362: F, t10595: F, t1547: F, t2799: F, t10599: F, t894: F, t1553: F, t2403: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13616, t13624, t13626, t13630, t13632, t13634, t13635) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1815::<F>(t13615, t901, t2815, t4370, t896, t2807, t4378, t2798, t4362, t10595, t1547, t2799);
        let (t13637, t13638, t13640, t13642) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1816::<F>(t10599, t1547, t2799, t13615, t894, t1553, t2403);
    (t13616, t13624, t13626, t13630, t13632, t13634, t13635, t13637, t13638, t13640, t13642)
}
