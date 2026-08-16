//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta891 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3079;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta891<F: Float>(t1071: F, t4743: F, t1078: F, t4772: F, t16237: F, t994: F, t11200: F, t1678: F, t3056: F, t4742: F, t378: F, t379: F) -> (F, F, F, F, F, F, F) {
        let (t53119, t53130, t53157, t53160, t53166, t53167, t53174) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3079::<F>(t1071, t4743, t1078, t4772, t16237, t994, t11200, t1678, t3056, t4742, t378, t379);
    (t53119, t53130, t53157, t53160, t53166, t53167, t53174)
}
