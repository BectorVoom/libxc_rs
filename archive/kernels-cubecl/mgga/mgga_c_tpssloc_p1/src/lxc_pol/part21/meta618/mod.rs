//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2394;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta618<F: Float>(t1369: F, t40059: F, t12345: F, t3876: F, t22843: F, t241: F, t67: F, t3872: F, t10021: F, t1336: F, t1339: F, t1354: F) -> (F, F, F, F, F, F) {
        let (t40060, t40065, t40070, t40079, t40123, t40124) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2394::<F>(t1369, t40059, t12345, t3876, t22843, t241, t67, t3872, t10021, t1336, t1339, t1354);
    (t40060, t40065, t40070, t40079, t40123, t40124)
}
