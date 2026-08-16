//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta229 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk987;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta229(t12987: f64, t480: f64, t1224: f64, t3362: f64, t12268: f64, t3698: f64, t3367: f64, t404: f64, t12256: f64, t11239: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t12988, t13006, t13020, t13026, t13027, t13036) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk987(t12987, t480, t1224, t3362, t12268, t3698, t3367, t404, t12256, t11239, t460);
    (t12988, t13006, t13020, t13026, t13027, t13036)
}
