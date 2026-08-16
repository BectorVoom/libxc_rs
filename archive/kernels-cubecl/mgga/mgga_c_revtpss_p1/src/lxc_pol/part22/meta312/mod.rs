//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta312 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1750;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1751;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta312<F: Float>(t58: F, t59: F, t10199: F, t2851: F, t78: F, t3361: F, t81: F, t116: F, t2319: F, t2389: F, t705: F, t2258: F, t750: F, t706: F, t157: F, t36: F, t2401: F, t200: F, t45: F, t202: F, t57: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10368, t10379, t10389, t10398, t10416) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1750::<F>(t58, t59, t10199, t2851, t78, t3361, t81, t116, t2319);
        let (t10428, t10436, t10437, t10439, t10443, t10446, t10457) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1751::<F>(t2389, t705, t2258, t750, t706, t157, t36, t2401, t200, t45, t202, t57);
    (t10368, t10379, t10389, t10398, t10416, t10428, t10436, t10437, t10439, t10443, t10446, t10457)
}
