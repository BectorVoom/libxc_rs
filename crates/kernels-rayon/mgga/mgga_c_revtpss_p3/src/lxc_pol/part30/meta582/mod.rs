//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2036;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta582(t25877: f64, t94390: f64, t94385: f64, t9675: f64, t7289: f64, t94377: f64, t122: f64, t72: f64, t7274: f64, t3916: f64, t25895: f64, t7285: f64, t9288: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94589, t94590, t94591, t94593, t94596, t94597, t94598, t94600) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2036(t25877, t94390, t94385, t9675, t7289, t94377, t122, t72, t7274, t3916, t25895, t7285, t9288);
    (t94589, t94590, t94591, t94593, t94596, t94597, t94598, t94600)
}
