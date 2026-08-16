//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta507 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1703;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta507<F: Float>(t25224: F, t7479: F, t6552: F, t23195: F, t5636: F, t6553: F, t1880: F, t5527: F, t6554: F, t23035: F, t16815: F, t232: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28288, t28289, t28294, t28295, t28296, t28298, t28299, t28300, t28321) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1703::<F>(t25224, t7479, t6552, t23195, t5636, t6553, t1880, t5527, t6554, t23035, t16815, t232);
    (t28288, t28289, t28294, t28295, t28296, t28298, t28299, t28300, t28321)
}
