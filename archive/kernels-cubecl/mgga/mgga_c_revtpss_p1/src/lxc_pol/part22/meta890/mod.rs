//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta890 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta890<F: Float>(t378: F, t53014: F, t1072: F, t994: F, t3046: F, t379: F, t11213: F, t1678: F, t16237: F, t342: F, t11120: F, t1695: F) -> (F, F, F, F, F, F) {
        let (t53015, t53027, t53034, t53058, t53093, t53108) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3078::<F>(t378, t53014, t1072, t994, t3046, t379, t11213, t1678, t16237, t342, t11120, t1695);
    (t53015, t53027, t53034, t53058, t53093, t53108)
}
