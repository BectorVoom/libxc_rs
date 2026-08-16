//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta523 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta523<F: Float>(t26223: F, t26364: F, t26485: F, t26500: F, t533: F, t1390: F, t1983: F, t16521: F, t1873: F, t16524: F, t7015: F, t5371: F, t6534: F) -> (F, F, F, F, F, F, F) {
        let (t26502, t26503, t26504, t26505, t26533, t26535, t26537) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1857::<F>(t26223, t26364, t26485, t26500, t533, t1390, t1983, t16521, t1873, t16524, t7015, t5371, t6534);
    (t26502, t26503, t26504, t26505, t26533, t26535, t26537)
}
