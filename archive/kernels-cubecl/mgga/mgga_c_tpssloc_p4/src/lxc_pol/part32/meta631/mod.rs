//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2043;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta631<F: Float>(t87291: F, t23062: F, t25106: F, t13176: F, t6613: F, t23133: F, t4257: F, t1496: F, t81942: F, t7497: F, t81933: F, t25098: F, t81835: F) -> (F, F, F, F, F, F, F) {
        let (t87292, t87293, t87295, t87301, t87304, t87306, t87308) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2043::<F>(t87291, t23062, t25106, t13176, t6613, t23133, t4257, t1496, t81942, t7497, t81933, t25098, t81835);
    (t87292, t87293, t87295, t87301, t87304, t87306, t87308)
}
