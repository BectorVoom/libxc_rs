//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta502 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1788;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1789;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta502(t1470: f64, t4173: f64, t1493: f64, t1497: f64, t77: f64, t5872: f64, t84: f64, t5819: f64, t603: f64, t5826: f64, t5816: f64, t30: f64, t5966: f64, t1544: f64, t1583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29538, t29543, t29544, t29547, t29548, t29551, t29554, t29561, t29562, t29591) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1788(t1470, t4173, t1493, t1497, t77, t5872, t84, t5819, t603, t5826, t5816, t30, t5966);
        let t29598 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1789(t1544, t1583);
    (t29538, t29543, t29544, t29547, t29548, t29551, t29554, t29561, t29562, t29591, t29598)
}
