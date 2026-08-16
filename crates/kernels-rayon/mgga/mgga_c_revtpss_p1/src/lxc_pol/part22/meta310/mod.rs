//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta310 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1748;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta310(t10276: f64, t22: f64, t2224: f64, t588: f64, t27: f64, t584: f64, t20: f64, t596: f64, t12: f64, t583: f64, t2231: f64, t2237: f64, t592: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10278, t10279, t10281, t10284, t10287, t10288, t10290) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1748(t10276, t22, t2224, t588, t27, t584, t20, t596, t12, t583, t2231, t2237, t592);
    (t10278, t10279, t10281, t10284, t10287, t10288, t10290)
}
