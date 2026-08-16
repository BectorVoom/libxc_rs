//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta197 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1246;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1247;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta197(t4719: f64, t983: f64, t1642: f64, t3022: f64, t1633: f64, t2986: f64, t974: f64, t981: f64, t4707: f64, t964: f64, t973: f64, t3011: f64, t3014: f64, t972: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4721, t4723, t4724, t4725, t4727, t4729, t4731, t4732) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1246(t4719, t983, t1642, t3022, t1633, t2986, t974, t981, t4707, t964, t973, t3011);
        let t4733 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1247(t3014, t972);
    (t4721, t4723, t4724, t4725, t4727, t4729, t4731, t4732, t4733)
}
