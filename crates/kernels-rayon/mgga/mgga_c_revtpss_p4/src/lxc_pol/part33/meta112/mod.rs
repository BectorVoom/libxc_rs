//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta112 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk665;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta112(t240: f64, t2681: f64, t243: f64, t247: f64, t237: f64, t124: f64, t212: f64, t596: f64, t800: f64) -> (f64, f64, f64, f64) {
        let (t2682, t2684, t2686, t2689) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk665(t240, t2681, t243, t247, t237, t124, t212, t596, t800);
    (t2682, t2684, t2686, t2689)
}
