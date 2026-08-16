//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2023;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta623<F: Float>(t86942: F, t23168: F, t25338: F, t23012: F, t7485: F, t25046: F, t6579: F, t1484: F, t2717: F, t225: F, t25051: F, t7489: F) -> (F, F, F, F, F, F, F) {
        let (t86943, t86951, t86955, t86968, t86969, t86988, t86991) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2023::<F>(t86942, t23168, t25338, t23012, t7485, t25046, t6579, t1484, t2717, t225, t25051, t7489);
    (t86943, t86951, t86955, t86968, t86969, t86988, t86991)
}
