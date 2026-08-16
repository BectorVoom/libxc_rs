//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta639 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2088;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta639(t98260: f64, t98285: f64, t98964: f64, t98976: f64, t98979: f64, t99009: f64, t99013: f64, t99035: f64, t99044: f64, t99050: f64, t99091: f64, t99113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t102549, t102569, t103264, t103269, t103270, t103285, t103287, t103297, t103302, t103305, t103329, t103347) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2088(t98260, t98285, t98964, t98976, t98979, t99009, t99013, t99035, t99044, t99050, t99091, t99113);
    (t102549, t102569, t103264, t103269, t103270, t103285, t103287, t103297, t103302, t103305, t103329, t103347)
}
