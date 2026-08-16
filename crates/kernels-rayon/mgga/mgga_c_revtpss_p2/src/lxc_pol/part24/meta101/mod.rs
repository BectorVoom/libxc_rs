//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta101 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk583;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk584;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta101(t1129: f64, t418: f64, t408: f64, t406: f64, t409: f64, t3356: f64, t281: f64, t2902: f64, t414: f64, t1224: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3382, t3383, t3384, t3390, t3394, t3402, t3407, t3413, t3414, t3417) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk583(t1129, t418, t408, t406, t409, t3356, t281, t2902, t414, t1224, t240);
        let t3431 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk584(t1129);
    (t3382, t3383, t3384, t3390, t3394, t3402, t3407, t3413, t3414, t3417, t3431)
}
