//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2061;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta600(t3650: f64, t7623: f64, t12881: f64, t7624: f64, t12854: f64, t29096: f64, t13089: f64, t12886: f64, t12948: f64, t26849: f64, t26852: f64, t3636: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t97138, t97141, t97149, t97154, t97161, t97169, t97171) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2061(t3650, t7623, t12881, t7624, t12854, t29096, t13089, t12886, t12948, t26849, t26852, t3636);
    (t97138, t97141, t97149, t97154, t97161, t97169, t97171)
}
