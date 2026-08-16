//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1987;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta599(t1329: f64, t80775: f64, t22822: f64, t281: f64, t6924: f64, t22794: f64, t120: f64, t22816: f64, t22814: f64, t22855: f64, t22823: f64, t3862: f64, t6940: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t80776, t80779, t80780, t80782, t80783, t80784, t80791, t80792, t80794) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1987(t1329, t80775, t22822, t281, t6924, t22794, t120, t22816, t22814, t22855, t22823, t3862, t6940);
    (t80776, t80779, t80780, t80782, t80783, t80784, t80791, t80792, t80794)
}
