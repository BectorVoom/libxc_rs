//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2099;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2100;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta649(t17361: f64, t7618: f64, t17289: f64, t2138: f64, t3666: f64, t8184: f64, t17451: f64, t26867: f64, t1285: f64, t97173: f64, t104646: f64, t17735: f64, t17617: f64, t26870: f64, t3682: f64, t8172: f64, t29020: f64, t3704: f64, t29086: f64, t3678: f64, t3655: f64, t8185: f64, t17628: f64, t7607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t104905, t104916, t104924, t104933, t104943, t104946) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2099(t17361, t7618, t17289, t2138, t3666, t8184, t17451, t26867, t1285, t97173, t104646, t17735);
        let (t104953, t104963, t104968, t104972, t104988, t104990) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2100(t17617, t26870, t3682, t8172, t29020, t3704, t29086, t3678, t3655, t8185, t17628, t7607);
    (t104905, t104916, t104924, t104933, t104943, t104946, t104953, t104963, t104968, t104972, t104988, t104990)
}
