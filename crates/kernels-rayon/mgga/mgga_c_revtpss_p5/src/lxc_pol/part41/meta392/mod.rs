//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta392 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1328;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta392(t1132: f64, t20337: f64, t1145: f64, t20318: f64, t141: f64, t20302: f64, t3417: f64, t20298: f64, t20310: f64, t20306: f64, t12327: f64, t6442: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t20338, t20341, t20344, t20347, t20350, t20353, t20356) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1328(t1132, t20337, t1145, t20318, t141, t20302, t3417, t20298, t20310, t20306, t12327, t6442);
    (t20338, t20341, t20344, t20347, t20350, t20353, t20356)
}
