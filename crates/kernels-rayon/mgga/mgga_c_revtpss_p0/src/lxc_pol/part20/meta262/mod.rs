//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta262 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1108;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta262(t3022: f64, t3026: f64, t11467: f64, t3011: f64, t973: f64, t981: f64, t2986: f64, t972: f64, t3007: f64, t11465: f64, t3014: f64, t11501: f64, t964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11596, t11598, t11600, t11601, t11602, t11604, t11606, t11608, t11610) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1108(t3022, t3026, t11467, t3011, t973, t981, t2986, t972, t3007, t11465, t3014, t11501, t964);
    (t11596, t11598, t11600, t11601, t11602, t11604, t11606, t11608, t11610)
}
