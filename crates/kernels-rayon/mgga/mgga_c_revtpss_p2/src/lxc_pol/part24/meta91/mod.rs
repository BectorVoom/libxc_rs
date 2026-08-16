//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta91 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk531;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk532;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk533;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk534;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta91(t2470: f64, t874: f64, t875: f64, t251: f64, t2718: f64, t1941: f64, t268: f64, t271: f64, t1065: f64, t159: f64, t631: f64, t2297: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2810, t2811, t2846, t2847, t2850) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk531(t2470, t874, t875, t251, t2718, t1941, t268, t271, t1065, t159);
        let t2851 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk532(t631);
        let t2852 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk533(t2851);
        let t2857 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk534(t2297);
    (t2810, t2811, t2846, t2847, t2850, t2851, t2852, t2857)
}
