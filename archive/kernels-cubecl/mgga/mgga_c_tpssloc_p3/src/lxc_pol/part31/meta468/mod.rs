//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1627;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta468<F: Float>(t4072: F, t88: F, t1453: F, t22470: F, t666: F, t22473: F, t4067: F, t6530: F, t1982: F, t8944: F, t1388: F, t1845: F) -> (F, F, F, F, F, F, F) {
        let (t26117, t26127, t26129, t26130, t26132, t26161, t26163) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1627::<F>(t4072, t88, t1453, t22470, t666, t22473, t4067, t6530, t1982, t8944, t1388, t1845);
    (t26117, t26127, t26129, t26130, t26132, t26161, t26163)
}
