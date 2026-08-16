//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta199 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk924;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta199(t1121: f64, t1263: f64, t1214: f64, t1469: f64, t1042: f64, t3362: f64, t3617: f64, t4181: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5296, t5297, t5298, t5299) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk924(t1121, t1263, t1214, t1469, t1042);
        let (t5302, t5303, t5304) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk925(t3362, t3617, t4181, t1042);
    (t5296, t5297, t5298, t5299, t5302, t5303, t5304)
}
