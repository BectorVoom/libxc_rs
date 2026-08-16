//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta328(t11249: f64, t13045: f64, t13044: f64, t1042: f64, t13040: f64, t3597: f64, t13036: f64, t3603: f64, t13032: f64, t3609: f64, t1244: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13047, t13048, t13051, t13052, t13054, t13055, t13058, t13061, t13062, t13063) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1244(t11249, t13045, t13044, t1042, t13040, t3597, t13036, t3603, t13032, t3609, t1244, t471);
    (t13047, t13048, t13051, t13052, t13054, t13055, t13058, t13061, t13062, t13063)
}
