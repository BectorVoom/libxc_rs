//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta77 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk461;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk462;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta77(t108: f64, t1509: f64, t105: f64, t109: f64, t1505: f64, t1507: f64, t97: f64, t114: f64, t655: f64, t653: f64, t69: f64) -> (f64, f64, f64) {
        let t1513 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk461(t108, t1509, t105, t109, t1505, t1507, t97);
        let (t1514, t1518) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk462(t114, t1513, t655, t653, t69);
    (t1513, t1514, t1518)
}
