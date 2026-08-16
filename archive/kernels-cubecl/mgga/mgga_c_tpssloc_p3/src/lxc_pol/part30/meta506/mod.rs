//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1825;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta506<F: Float>(t1597: F, t40: F, t1933: F, t23479: F, t1015: F, t7582: F, t23472: F, t343: F, t23562: F, t23509: F, t3: F, t23470: F, t3030: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25637, t25638, t25639, t25641, t25642, t25644, t25645, t25650, t25651) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1825::<F>(t1597, t40, t1933, t23479, t1015, t7582, t23472, t343, t23562, t23509, t3, t23470, t3030);
    (t25637, t25638, t25639, t25641, t25642, t25644, t25645, t25650, t25651)
}
