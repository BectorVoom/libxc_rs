//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta199 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk945;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta199<F: Float>(t300: F, t4479: F, t4447: F, t1573: F, t961: F, t1589: F, t2940: F, t1580: F, t2904: F, t952: F, t959: F, t4471: F, t942: F, t951: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4480, t4482, t4483, t4485, t4487, t4488, t4489, t4491, t4493) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk945::<F>(t300, t4479, t4447, t1573, t961, t1589, t2940, t1580, t2904, t952, t959, t4471, t942, t951);
    (t4480, t4482, t4483, t4485, t4487, t4488, t4489, t4491, t4493)
}
