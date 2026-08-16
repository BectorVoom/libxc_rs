//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta231 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk989;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta231(t11249: f64, t13045: f64, t13040: f64, t3597: f64, t13036: f64, t3603: f64, t1244: f64, t471: f64, t3367: f64, t414: f64, t66: f64, t11239: f64, t1243: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13046, t13051, t13052, t13053, t13061, t13062, t13063, t13099, t13100, t13126) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk989(t11249, t13045, t13040, t3597, t13036, t3603, t1244, t471, t3367, t414, t66, t11239, t1243);
    (t13046, t13051, t13052, t13053, t13061, t13062, t13063, t13099, t13100, t13126)
}
