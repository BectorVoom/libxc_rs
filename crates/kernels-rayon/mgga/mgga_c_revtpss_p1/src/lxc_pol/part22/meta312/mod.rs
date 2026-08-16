//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta312 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1750;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1751;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta312(t58: f64, t59: f64, t10199: f64, t2851: f64, t78: f64, t3361: f64, t81: f64, t116: f64, t2319: f64, t2389: f64, t705: f64, t2258: f64, t750: f64, t706: f64, t157: f64, t36: f64, t2401: f64, t200: f64, t45: f64, t202: f64, t57: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10368, t10379, t10389, t10398, t10416) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1750(t58, t59, t10199, t2851, t78, t3361, t81, t116, t2319);
        let (t10428, t10436, t10437, t10439, t10443, t10446, t10457) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1751(t2389, t705, t2258, t750, t706, t157, t36, t2401, t200, t45, t202, t57);
    (t10368, t10379, t10389, t10398, t10416, t10428, t10436, t10437, t10439, t10443, t10446, t10457)
}
