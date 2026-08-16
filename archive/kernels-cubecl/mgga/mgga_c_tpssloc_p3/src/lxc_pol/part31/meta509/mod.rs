//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1705;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta509<F: Float>(t28337: F, t6646: F, t22986: F, t5527: F, t6638: F, t6637: F, t23035: F, t1484: F, t25319: F, t6552: F, t5612: F, t815: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28338, t28339, t28341, t28342, t28343, t28345, t28346, t28347, t28356) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1705::<F>(t28337, t6646, t22986, t5527, t6638, t6637, t23035, t1484, t25319, t6552, t5612, t815);
    (t28338, t28339, t28341, t28342, t28343, t28345, t28346, t28347, t28356)
}
