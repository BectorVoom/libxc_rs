//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta261 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1401;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1402;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1403;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta261<F: Float>(t10213: F, t974: F, t2769: F, t632: F, t344: F, t9288: F, t698: F, t976: F, t979: F, t973: F, t2970: F, t2999: F, t135: F, t2978: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10214, t10216) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1401::<F>(t10213, t974, t2769, t632);
        let (t10218, t10219, t10224) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1402::<F>(t10216, t344, t9288, t10214, t698, t976);
        let (t10225, t10226, t10228, t10229, t10231) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1403::<F>(t10224, t979, t973, t2970, t2999, t135, t2978);
    (t10214, t10216, t10218, t10219, t10224, t10225, t10226, t10228, t10229, t10231)
}
