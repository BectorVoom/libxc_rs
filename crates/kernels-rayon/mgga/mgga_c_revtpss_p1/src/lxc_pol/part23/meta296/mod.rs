//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta296 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1538;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1539;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1540;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1541;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta296(t126: f64, t3181: f64, t1003: f64, t3080: f64, t221: f64, t346: f64, t68: f64, t345: f64, t1014: f64, t2852: f64, t245: f64, t3089: f64, t3088: f64, t3114: f64, t11223: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11725, t11732, t11735, t11737, t11765, t11772) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1538(t126, t3181, t1003, t3080, t221, t346, t68, t345, t1014, t2852, t245, t3089);
        let t11773 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1539(t11772, t3088);
        let t11774 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1540(t11773, t3114);
        let t11788 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1541(t11223, t225);
    (t11725, t11732, t11735, t11737, t11765, t11772, t11773, t11774, t11788)
}
