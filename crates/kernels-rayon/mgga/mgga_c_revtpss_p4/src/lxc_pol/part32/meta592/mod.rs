//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta592(t98968: f64, t98972: f64, t98983: f64, t98991: f64, t99000: f64, t99006: f64, t99011: f64, t99019: f64, t99021: f64, t99023: f64, t99026: f64, t99029: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t103265, t103267, t103273, t103276, t103280, t103283, t103286, t103290, t103291, t103292, t103293, t103294) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1924(t98968, t98972, t98983, t98991, t99000, t99006, t99011, t99019, t99021, t99023, t99026, t99029);
    (t103265, t103267, t103273, t103276, t103280, t103283, t103286, t103290, t103291, t103292, t103293, t103294)
}
