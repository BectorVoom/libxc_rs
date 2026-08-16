//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta163 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk797;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk798;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta163<F: Float>(t300: F, t4479: F, t4447: F, t1573: F, t961: F, t1589: F, t2940: F, t1580: F, t2904: F, t952: F, t959: F, t4471: F, t942: F, t951: F, t2929: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4480, t4482, t4483) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk797::<F>(t300, t4479, t4447, t1573);
        let (t4485, t4487, t4488, t4489, t4491, t4493, t4495, t4496) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk798::<F>(t4483, t961, t1589, t2940, t1580, t2904, t952, t959, t4471, t942, t951, t2929);
    (t4480, t4482, t4483, t4485, t4487, t4488, t4489, t4491, t4493, t4495, t4496)
}
