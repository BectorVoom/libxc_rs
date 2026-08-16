//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta312 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1315;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta312(t2389: f64, t705: f64, t2258: f64, t750: f64, t706: f64, t157: f64, t36: f64, t2401: f64, t200: f64, t45: f64, t202: f64, t57: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t10428, t10437, t10439, t10443, t10446, t10457) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1315(t2389, t705, t2258, t750, t706, t157, t36, t2401, t200, t45, t202, t57);
    (t10428, t10437, t10439, t10443, t10446, t10457)
}
