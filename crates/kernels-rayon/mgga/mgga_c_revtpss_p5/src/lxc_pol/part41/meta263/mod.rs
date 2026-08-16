//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1005;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta263(t3869: f64, t9572: f64, t2434: f64, t762: f64, t1331: f64, t3860: f64, t186: f64, t685: f64, t793: f64, t1337: f64, t4146: f64, t565: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t9574, t9575, t9577, t9578, t9586, t9588, t9593) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1005(t3869, t9572, t2434, t762, t1331, t3860, t186, t685, t793, t1337, t4146, t565);
    (t9574, t9575, t9577, t9578, t9586, t9588, t9593)
}
