//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta750 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2627;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta750(t47099: f64, t47101: f64, t13665: f64, t9575: f64, t47106: f64, t47110: f64, t47113: f64, t47119: f64, t47125: f64, t47127: f64, t40067: f64, t40072: f64, t47109: f64, t47116: f64, t47118: f64, t47122: f64, t47124: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48311, t48312, t48314, t48315, t48316, t48317, t48318, t48319, t48320, t48321) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2627(t47099, t47101, t13665, t9575, t47106, t47110, t47113, t47119, t47125, t47127, t40067, t40072, t47109, t47116, t47118, t47122, t47124);
    (t48311, t48312, t48314, t48315, t48316, t48317, t48318, t48319, t48320, t48321)
}
