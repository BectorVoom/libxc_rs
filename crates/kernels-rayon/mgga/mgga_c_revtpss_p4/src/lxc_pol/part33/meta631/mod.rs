//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta631(t7058: f64, t99321: f64, t7759: f64, t822: f64, t25310: f64, t27279: f64, t27186: f64, t93321: f64, t93374: f64, t122: f64, t72: f64, t2466: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t99323, t99334, t99342, t99344, t99346, t99348, t99349) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2078(t7058, t99321, t7759, t822, t25310, t27279, t27186, t93321, t93374, t122, t72, t2466);
    (t99323, t99334, t99342, t99344, t99346, t99348, t99349)
}
