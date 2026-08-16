//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta204 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk938;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta204(t760: f64, t9419: f64, t9387: f64, t9372: f64, t9425: f64, t2475: f64, t73: f64, t2710: f64, t2793: f64, t9285: f64, t874: f64, t875: f64, t9288: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10592, t10596, t10604, t10611, t10626, t10645, t10651) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk938(t760, t9419, t9387, t9372, t9425, t2475, t73, t2710, t2793, t9285, t874, t875, t9288);
    (t10592, t10596, t10604, t10611, t10626, t10645, t10651)
}
