//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta41 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk282;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta41<F: Float>(t880: F, t307: F, t302: F, t906: F, t310: F, t320: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t917, t922, t923, t924, t926, t929, t932) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk282::<F>(t880, t307, t302, t906, t310);
        let (t936, t941, t942) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk283::<F>(t880, t320);
    (t917, t922, t923, t924, t926, t929, t932, t936, t941, t942)
}
